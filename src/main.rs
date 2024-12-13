use rusqlite::{Connection, Result};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};


struct Spool {
    roll_id: Option<i32>,
    roll_name: Option<String>,
    roll_weight: Option<i32>,
    roll_length: Option<i32>,
    timestamp: Option<i64>,
}

impl Spool {
    fn get_weight(&mut self) -> i32 {
        const CONVERSION_FACTOR: f32 = 3.0303;
        match self.roll_weight {
            Some(val) => val,
            None => {
                let length = match self.roll_length {
                    Some(val) => val as f32,
                    None => panic!("No Vals Set")
                };
                let weight = length * CONVERSION_FACTOR;
                self.roll_weight = Some(weight as i32);
                weight as i32
            }
        }
    }
}

struct Filament {}

fn main() {

    let db_path = "./3d_print.db";
    let db = Connection::open(db_path).unwrap();
    println!("Connection to database has been established");
    create_new_spool_tbl(&db).unwrap();

    //println!("Remainder 3D Printing");
    //println!("Create New Real");
    //println!("Add Print");
    //println!("Check Remaining");

    let rt = db.close();
    rt.unwrap();
}

fn check_remaining() -> u32 {
    return 0;
}

fn add_new_print(print_weight: Option<f32>, print_length: Option<f32>) {}

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
                roll_weight INTEGER,
                roll_length INTEGER,
                roll_timestamp INTEGER NOT NULL)";
            conn.execute(create_query, ()).unwrap();
            println!("Created Spool Table");
        }
        1 => {
            println!("Spool table found")
        }
        _ => {
            println!("Issue with finding table");
            return Err("Invalid Amount of tables")
        }
    }
    Ok(())
}

fn create_new_filament_tbl(conn: &Connection) -> Result<(), &'static str> {
    let check_query = "SELECT count(name) FROM sqlite_master WHERE type='table' AND name='filament'";
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
                print_time INT,
                roll_id INTEGER NOT NULL)";
            conn.execute(create_query, ()).unwrap();
            println!("Created filament Table");
        }
        1 => {
            println!("Filament table found")
        }
        _ => {
            println!("Issue with finding table");
            return Err("Invalid Amount of tables")
        }
    }
    Ok(())
}
//Function to get the current timestamp
pub fn get_timestamp() -> i64 {
    let start = SystemTime::now();
    let timestamp_current = i64::try_from(start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs()).unwrap();
    timestamp_current
} 

fn open_new_real(conn: &Connection, mut spool_info: Spool) {
    let roll_id = Uuid::new_v4();
    conn.execute(
    "INSERT INTO spool (roll_id,
                        roll_name,
                        roll_weight,
                        roll_length,
                        roll_timestamp)
            VALUES (?1,?2,?3,?4,?5)", 
        (&roll_id.as_bytes(),
        spool_info.get_weight(),
    )
    ).unwrap();

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Test_Spool_Creation() {
        let conn = Connection::open_in_memory().unwrap();
        create_new_spool_tbl(&conn).unwrap();
        create_new_spool_tbl(&conn).unwrap();
    }

    #[test]
    fn Test_filament_Creation() {
        let conn = Connection::open_in_memory().unwrap();
        create_new_filament_tbl(&conn).unwrap();
        create_new_filament_tbl(&conn).unwrap();
    }

    #[test]
    fn Test_spool_weight_1() {
        let mut test_spool = Spool {
            roll_id: Some(1),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000),
            roll_length: None,
            timestamp: Some(get_timestamp())
        };

        let ans = test_spool.get_weight();
        assert_eq!(ans, 1000);
    }

    #[test]
    fn Test_spool_weight_2() {
        let mut test_spool = Spool {
            roll_id: Some(1),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: None,
            roll_length: Some(330),
            timestamp: Some(get_timestamp())
        };

        let ans = test_spool.get_weight();
        assert_eq!(ans, 999);
    }
}
