use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
mod tbl_creation;
mod print_add;
mod print_stats;
mod print_structs;
use print_structs::*;

/// CLI to keep track and know levels of a 3D printers filament levels
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
    CheckRemaining,
    LifetimeStats,
}


fn main() {
    let db_path = "./3d_print.db";
    let db = Connection::open(db_path).unwrap();
    println!("Connection to database has been established");
    //Create test spool
    tbl_creation::create_new_spool_tbl(&db).unwrap();
    tbl_creation::create_new_filament_tbl(&db).unwrap();
    let args = Args::parse();

    match args.cmd {
        Commands::AddPrint { print_time } => {
            println!("Adding New Print: Print Time {}", print_time);
            let mut new_print = Filament {
                print_id: Some(Uuid::new_v4()),
                print_weight: args.weight,
                print_length: args.length,
                print_time: Some(print_time),
                roll_id: None,
            };

            let print_rt = print_add::add_new_print(&db, &mut new_print).unwrap();
            if print_rt != 1 {
                panic!("Didnt Successfully Create Print");
            }
        }
        Commands::CreateSpool { spool_name } => {
            println!("Creating New spool: {}", spool_name);
            let mut new_spool = Spool {
                roll_id: Some(Uuid::new_v4()),
                roll_name: Some(spool_name),
                roll_weight: args.weight,
                roll_length: args.length,
                timestamp: Some(get_timestamp()),
            };
            let spool_rt = print_add::open_new_spool(&db, &mut new_spool).unwrap();
            if spool_rt != 1 {
                panic!("Didnt Successfully Create Spool");
            }
        }
        Commands::CheckRemaining => {
            println!("Checking Remaining levels of Printer");
            let (weight, length) = print_stats::check_remaining(&db);
            println!("Estimated REMAINING Weight: {} gram", weight);
            println!("Estimated REMAINING Lenght: {} meters", length);
        }
        Commands::LifetimeStats => {
            println!("Lifetime Stats for printer:");
            let (total_weight, total_length, total_time) = print_stats::lifetime_statistics(&db);
            println!("Total Amount of Fillament used: {} grams", total_weight);
            println!("Total Length of Fillament used: {} meters", total_length);
            let time_converted = total_time/60 as i32;
            println!("Total Printing Time: {} min", time_converted);

        }
    }

    let rt = db.close();
    rt.unwrap();
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
        tbl_creation::create_new_spool_tbl(&conn).unwrap();
        tbl_creation::create_new_spool_tbl(&conn).unwrap();
    }

    #[test]
    fn test_filament_tbl_creation() {
        let conn = Connection::open_in_memory().unwrap();
        tbl_creation::create_new_filament_tbl(&conn).unwrap();
        tbl_creation::create_new_filament_tbl(&conn).unwrap();
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
        tbl_creation::create_new_spool_tbl(&conn).unwrap();
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp()),
        };
        let rt = print_add::open_new_spool(&conn, &mut test_spool).unwrap();
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
        tbl_creation::create_new_spool_tbl(&conn).unwrap();
        tbl_creation::create_new_filament_tbl(&conn).unwrap();

        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp()),
        };
        let rt = print_add::open_new_spool(&conn, &mut test_spool).unwrap();
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
        let rt_second_spool = print_add::open_new_spool(&conn, &mut second_test_spool).unwrap();
        assert_eq!(rt_second_spool, 1);

        //Test print creation
        let mut test_print = Filament {
            print_id: Some(Uuid::new_v4()),
            print_weight: None,
            print_length: Some(2.31),
            print_time: Some(1125),
            roll_id: None,
        };

        let _rt2 = print_add::add_new_print(&conn, &mut test_print).unwrap();
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
        tbl_creation::create_new_spool_tbl(&conn).unwrap();
        tbl_creation::create_new_filament_tbl(&conn).unwrap();

        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp()),
        };
        let rt = print_add::open_new_spool(&conn, &mut test_spool).unwrap();
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
        let _rt2 = print_add::add_new_print(&conn, &mut test_print).unwrap();
        let ans = print_stats::check_remaining(&conn);
        assert_eq!(ans.0, 993.0);
        assert_eq!(ans.1, 327.69);

        let _rt3 = print_add::add_new_print(&conn, &mut second_test_print).unwrap();
        let ans = print_stats::check_remaining(&conn);
        assert_eq!(ans.0, 903.4);
        assert_eq!(ans.1, 298.122);
    }

    #[test]
    fn test_check_lifetime() {
        //Create in memory DB
        let conn = Connection::open_in_memory().unwrap();

        //Create test spool
        tbl_creation::create_new_spool_tbl(&conn).unwrap();
        tbl_creation::create_new_filament_tbl(&conn).unwrap();

        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000.0),
            roll_length: Some(330.0),
            timestamp: Some(get_timestamp()),
        };
        let rt = print_add::open_new_spool(&conn, &mut test_spool).unwrap();
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
        let _rt2 = print_add::add_new_print(&conn, &mut test_print).unwrap();
        let _rt3 = print_add::add_new_print(&conn, &mut second_test_print).unwrap();
        let ans = print_stats::lifetime_statistics(&conn);
        assert_eq!(ans.0, 96.59999);
        assert_eq!(ans.1, 31.878);
        assert_eq!(ans.2, 3825);
    }
}
