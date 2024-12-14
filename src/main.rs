use rusqlite::{config::DbConfig, Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,

    /// The weight of the spool or the print.
    #[arg(short, long)]
    weight: Option<f32>,
    
    /// The lenght of the spool or the print.
    #[arg(short, long)]
    length: Option<f32>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    CreateSpool {
        /// Spools Name
        spool_name: String,
    },
    AddPrint {
        print_time: i32,
    }, 
    CheckRemaing,
}

#[derive(Debug)]
struct Spool {
    roll_id: Option<Uuid>,
    roll_name: Option<String>,
    roll_weight: Option<f32>,
    roll_length: Option<f32>,
    timestamp: Option<i64>,
}

#[derive(Debug)]
struct Filament {
    print_id: Option<Uuid>,
    print_weight: Option<f32>,
    print_length: Option<f32>,
    print_time: Option<i32>,
    roll_id: Option<Uuid>,
}

#[derive(Debug)]
struct Remaining(f32, f32);

impl Spool {
    fn get_weight(&mut self) -> f32 {
        const CONVERSION_FACTOR: f32 = 3.0303;
        match self.roll_weight {
            Some(val) => val,
            None => {
                let length = match self.roll_length {
                    Some(val) => val as f32,
                    None => panic!("No Vals Set"),
                };
                let weight = length * CONVERSION_FACTOR;
                self.roll_weight = Some(weight);
                weight
            }
        }
    }

    fn get_length(&mut self) -> f32 {
        const CONVERSION_FACTOR: f32 = 0.33;
        match self.roll_length {
            Some(val) => val,
            None => {
                let weight = self.roll_weight.unwrap() as f32;
                let length = weight * CONVERSION_FACTOR;
                self.roll_length = Some(length);
                length
            }
        }
    }
}

impl Filament {
    fn get_weight(&mut self) -> f32 {
        const CONVERSION_FACTOR: f32 = 3.0303;
        match self.print_weight {
            Some(val) => val,
            None => {
                let length = self.print_length.unwrap();
                let weight = length * CONVERSION_FACTOR;
                self.print_weight = Some(weight);
                weight
            }
        }
    }

    fn get_length(&mut self) -> f32 {
        const CONVERSION_FACTOR: f32 = 0.33;
        match self.print_length {
            Some(val) => val,
            None => {
                let weight = self.print_weight.unwrap();
                let length = weight * CONVERSION_FACTOR;
                self.print_length = Some(length);
                length
            }
        }
    }
}

fn main() {
    let db_path = "./3d_print.db";
    let db = Connection::open(db_path).unwrap();
    println!("Connection to database has been established");
    let args = Args::parse();
  
    match args.cmd {
        Commands::AddPrint{print_time} => {
            println!("Adding New Print: Print Time {}", print_time);
            let mut new_print = Filament {
                print_id: Some(Uuid::new_v4()),
                print_weight: args.weight,
                print_length: args.length,
                print_time: Some(print_time),
                roll_id: None,
            };

            let print_rt = add_new_print(&db, &mut new_print).unwrap();
            if print_rt != 1 {
                panic!("Didnt Successfully Create Print");
            }
        },
        Commands::CreateSpool{spool_name} => {
            println!("Creating New spool: {}", spool_name);
            let mut new_spool = Spool {
                roll_id: Some(Uuid::new_v4()),
                roll_name: Some(spool_name),
                roll_weight: args.weight,
                roll_length: args.length,
                timestamp: Some(get_timestamp()),
            };
            let rt = open_new_spool(&db, &mut new_spool).unwrap();
            assert_eq!(rt, 1);

        },
        Commands::CheckRemaing => println!("Checking Remaining"),
        _ => panic!("Something went wrong selecting commands"),
    }

    let rt = db.close();
    rt.unwrap();
}

fn check_remaining(conn: &Connection) -> (f32, f32) {
    //Get Spool currently used
    let current_spool = get_current_spool(conn).unwrap();

    //Get the sum of weight and length for current spool.
    //Get information for spool.
    //Minus sum from original for remaining
    let accu_query = "SELECT SUM(print_weight), SUM(print_length) FROM filament WHERE roll_id = ?1";
    let accu_rt = conn
        .query_row(accu_query, [current_spool.roll_id], |row| {
            Ok(Filament {
                print_id: None,
                print_weight: row.get(0)?,
                print_length: row.get(1)?,
                print_time: None,
                roll_id: None,
            })
        })
        .unwrap();
    let original_query = "SELECT roll_weight, roll_length FROM spool WHERE roll_id = ?1";
    let original_rt = conn
        .query_row(original_query, [current_spool.roll_id], |row| {
            Ok(Spool {
                roll_id: None,
                roll_name: None,
                roll_weight: row.get(0)?,
                roll_length: row.get(1)?,
                timestamp: None,
            })
        })
        .unwrap();

    let remaining_length =
        original_rt.roll_length.unwrap() - accu_rt.print_length.unwrap();
    let remaining_weight =
        original_rt.roll_weight.unwrap() - accu_rt.print_weight.unwrap();
    let remaining_rt = (remaining_weight, remaining_length);
    remaining_rt
}

struct RollId {
    roll_id: Uuid,
}

fn get_current_spool(conn: &Connection) -> Result<RollId> {
    let check_query =
        "SELECT r.roll_id FROM spool r WHERE r.roll_timestamp = (SELECT MAX(roll_timestamp) FROM spool)";

    let exists_rt = conn.query_row(check_query, [], |row| {
        Ok(RollId {
            roll_id: row.get(0).unwrap(),
        })
    });
    exists_rt
}

fn add_new_print(conn: &Connection, print: &mut Filament) -> Result<usize> {
    //Get Spool currently used
    let exists_rt = get_current_spool(conn).unwrap();
    print.roll_id = Some(exists_rt.roll_id);

    //Add print to list
    let rt = conn.execute(
        "INSERT INTO filament (print_id,
                        print_weight,
                        print_length,
                        print_time,
                        roll_id)
            VALUES (?1,?2,?3,?4,?5)",
        (
            &print.print_id.unwrap().as_bytes(),
            print.get_weight(),
            print.get_length(),
            print.print_time,
            &print.roll_id.unwrap().as_bytes(),
        ),
    );
    println!("New print created");
    return rt;
}

fn open_new_spool(conn: &Connection, spool_info: &mut Spool) -> Result<usize> {
    let rt = conn.execute(
        "INSERT INTO spool (roll_id,
                        roll_name,
                        roll_weight,
                        roll_length,
                        roll_timestamp)
            VALUES (?1,?2,?3,?4,?5)",
        (
            &spool_info.roll_id.unwrap().as_bytes(),
            spool_info.roll_name.clone(),
            spool_info.get_weight(),
            spool_info.get_length(),
            spool_info.timestamp,
        ),
    );
    println!("New spool created");
    return rt;
}

fn create_new_spool_tbl(conn: &Connection) -> Result<(), &'static str> {
    let check_query = "SELECT count(name) FROM sqlite_master WHERE type='table' AND name='spool'";
    let exists_rt = conn.query_row(check_query, [], |row| row.get(0));
    let exists = match exists_rt {
        Ok(exists) => exists,
        Err(e) => {
            eprintln!("Err: {}", e);
            return Err("Err with query");
        }
    };
    match exists {
        0 => {
            println!("No spool table found, need to create one");
            let create_query = "CREATE TABLE spool (
                roll_id BLOB PRIMARY KEY,
                roll_name TEXT,
                roll_weight REAL,
                roll_length REAL,
                roll_timestamp INTEGER NOT NULL)";
            conn.execute(create_query, ()).unwrap();
            println!("Created Spool Table");
        }
        1 => {
            println!("Spool table found")
        }
        _ => {
            println!("Issue with finding table");
            return Err("Invalid Amount of tables");
        }
    }
    Ok(())
}

fn create_new_filament_tbl(conn: &Connection) -> Result<(), &'static str> {
    let check_query =
        "SELECT count(name) FROM sqlite_master WHERE type='table' AND name='filament'";
    let exists_rt = conn.query_row(check_query, [], |row| row.get(0));
    let exists = match exists_rt {
        Ok(exists) => exists,
        Err(e) => {
            eprintln!("Err: {}", e);
            return Err("Err with query");
        }
    };
    match exists {
        0 => {
            println!("No filament table found, need to create one");
            let create_query = "CREATE TABLE filament(
                print_id BLOB PRIMARY KEY,
                print_weight REAL,
                print_length REAL,
                print_time INTEGER,
                roll_id BLOB NOT NULL)";
            conn.execute(create_query, ()).unwrap();
            println!("Created filament Table");
        }
        1 => {
            println!("Filament table found")
        }
        _ => {
            println!("Issue with finding table");
            return Err("Invalid Amount of tables");
        }
    }
    Ok(())
}
//Function to get the current timestamp
pub fn get_timestamp() -> i64 {
    let start = SystemTime::now();
    let timestamp_current = i64::try_from(
        start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
    )
    .unwrap();
    timestamp_current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spool_tbl_creation() {
        let conn = Connection::open_in_memory().unwrap();
        create_new_spool_tbl(&conn).unwrap();
        create_new_spool_tbl(&conn).unwrap();
    }

    #[test]
    fn test_filament_tbl_creation() {
        let conn = Connection::open_in_memory().unwrap();
        create_new_filament_tbl(&conn).unwrap();
        create_new_filament_tbl(&conn).unwrap();
    }

    #[test]
    fn test_spool_weight_1() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: None,
            timestamp: Some(get_timestamp()),
        };

        let ans = test_spool.get_weight();
        assert_eq!(ans, 1000.0);
    }

    #[test]
    fn test_spool_weight_2() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: None,
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp()),
        };

        let ans = test_spool.get_weight();
        assert_eq!(ans, 999.99896);
    }

    #[test]
    #[should_panic]
    fn test_spool_weight_none() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: None,
            roll_length: None,
            timestamp: Some(get_timestamp()),
        };

        let _ans = test_spool.get_weight();
    }

    #[test]
    fn test_spool_length_1() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: None,
            timestamp: Some(get_timestamp()),
        };

        let ans = test_spool.get_length();
        assert_eq!(ans, 330.0);
    }

    #[test]
    fn test_spool_length_2() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: None,
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp()),
        };

        let ans = test_spool.get_length();
        assert_eq!(ans, 330.0);
    }

    #[test]
    #[should_panic]
    fn test_spool_length_none() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: None,
            roll_length: None,
            timestamp: Some(get_timestamp()),
        };

        let _ans = test_spool.get_length();
    }

    #[test]
    fn test_create_new_spool() {
        let conn = Connection::open_in_memory().unwrap();
        create_new_spool_tbl(&conn).unwrap();
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp()),
        };
        let rt = open_new_spool(&conn, &mut test_spool).unwrap();
        assert_eq!(rt, 1);
        let check_query = "SELECT * FROM spool";
        let exists_rt = conn
            .query_row(check_query, [], |row| {
                Ok(Spool {
                    roll_id: row.get(0)?,
                    roll_name: row.get(1)?,
                    roll_weight: row.get(2)?,
                    roll_length: row.get(3)?,
                    timestamp: row.get(4)?,
                })
            })
            .unwrap();

        assert_eq!(exists_rt.roll_id.unwrap(), test_spool.roll_id.unwrap());
        assert_eq!(exists_rt.roll_name.unwrap(), "crealtivity".to_string());
        assert_eq!(exists_rt.roll_weight.unwrap(), 1000.0);
        assert_eq!(exists_rt.roll_length.unwrap(), 330.0);
        assert!(exists_rt.timestamp.unwrap() > 1734209754);
    }

    #[test]
    fn test_create_new_print() {
        //Create in memory DB
        let conn = Connection::open_in_memory().unwrap();

        //Create test spool
        create_new_spool_tbl(&conn).unwrap();
        create_new_filament_tbl(&conn).unwrap();

        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp()),
        };
        let rt = open_new_spool(&conn, &mut test_spool).unwrap();
        assert_eq!(rt, 1);
        let check_query = "SELECT * FROM spool";
        let exists_rt = conn
            .query_row(check_query, [], |row| {
                Ok(Spool {
                    roll_id: row.get(0)?,
                    roll_name: row.get(1)?,
                    roll_weight: row.get(2)?,
                    roll_length: row.get(3)?,
                    timestamp: row.get(4)?,
                })
            })
            .unwrap();

        assert_eq!(exists_rt.roll_id.unwrap(), test_spool.roll_id.unwrap());
        assert_eq!(exists_rt.roll_name.unwrap(), "crealtivity".to_string());
        assert_eq!(exists_rt.roll_weight.unwrap(), 1000.0);
        assert_eq!(exists_rt.roll_length.unwrap(), 330.0);
        assert!(exists_rt.timestamp.unwrap() > 1734209754);
        let mut second_test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp() + 5),
        };
        let rt_second_spool = open_new_spool(&conn, &mut second_test_spool).unwrap();
        assert_eq!(rt_second_spool, 1);

        //Test print creation
        let mut test_print = Filament {
            print_id: Some(Uuid::new_v4()),
            print_weight: None,
            print_length: Some(2.31),
            print_time: Some(1125),
            roll_id: None,
        };

        let _rt2 = add_new_print(&conn, &mut test_print).unwrap();
        let check_query = "SELECT * FROM filament";
        let exists_rt2 = conn
            .query_row(check_query, [], |row| {
                Ok(Filament {
                    print_id: row.get(0)?,
                    print_weight: row.get(1)?,
                    print_length: row.get(2)?,
                    print_time: row.get(3)?,
                    roll_id: row.get(4)?,
                })
            })
            .unwrap();

        assert_eq!(exists_rt2.print_id.unwrap(), test_print.print_id.unwrap());
        assert_eq!(exists_rt2.print_weight.unwrap(), 6.9999924);
        assert_eq!(exists_rt2.print_length.unwrap(), 2.31);
        assert_eq!(exists_rt2.print_time.unwrap(), 1125);
        assert_eq!(
            exists_rt2.roll_id.unwrap(),
            second_test_spool.roll_id.unwrap()
        );
    }

    #[test]
    fn test_create_remaining() {
        //Create in memory DB
        let conn = Connection::open_in_memory().unwrap();

        //Create test spool
        create_new_spool_tbl(&conn).unwrap();
        create_new_filament_tbl(&conn).unwrap();

        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp()),
        };
        let rt = open_new_spool(&conn, &mut test_spool).unwrap();
        assert_eq!(rt, 1);
        //Test print creation
        let mut test_print = Filament {
            print_id: Some(Uuid::new_v4()),
            print_weight: None,
            print_length: Some(2.31),
            print_time: Some(1125),
            roll_id: None,
        };

        let mut second_test_print = Filament {
            print_id: Some(Uuid::new_v4()),
            print_weight: Some(89.6),
            print_length: None,
            print_time: Some(2700),
            roll_id: None,
        };
        let _rt2 = add_new_print(&conn, &mut test_print).unwrap();
        let ans = check_remaining(&conn);
        assert_eq!(ans.0, 993.0);
        assert_eq!(ans.1, 327.69);

        let _rt3 = add_new_print(&conn, &mut second_test_print).unwrap();
        let ans = check_remaining(&conn);
        assert_eq!(ans.0, 903.4);
        assert_eq!(ans.1, 298.122);
    }
}
