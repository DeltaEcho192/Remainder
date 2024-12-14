use rusqlite::{Connection, Result};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};


struct Spool {
    roll_id: Option<Uuid>,
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

    fn get_length(&mut self) -> i32 {
        const CONVERSION_FACTOR: f32 = 0.33;
        match self.roll_length { 
            Some(val) => val,
            None => {
                let weight = self.roll_weight.unwrap() as f32;
                let length = weight * CONVERSION_FACTOR;
                self.roll_length = Some(length as i32);
                length as i32
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
    )
    );
    println!("New spool created");
    return rt;
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
            roll_weight: Some(1000),
            roll_length: None,
            timestamp: Some(get_timestamp())
        };

        let ans = test_spool.get_weight();
        assert_eq!(ans, 1000);
    }

    #[test]
    fn test_spool_weight_2() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: None,
            roll_length: Some(330),
            timestamp: Some(get_timestamp())
        };

        let ans = test_spool.get_weight();
        assert_eq!(ans, 999);
    }

    #[test]
    #[should_panic]
    fn test_spool_weight_none() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: None,
            roll_length: None,
            timestamp: Some(get_timestamp())
        };

        let _ans = test_spool.get_weight();
    }

    #[test]
    fn test_spool_length_1() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: Some(1000),
            roll_length: None,
            timestamp: Some(get_timestamp())
        };

        let ans = test_spool.get_length();
        assert_eq!(ans, 330);
    }

    #[test]
    fn test_spool_length_2() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: None,
            roll_length: Some(330),
            timestamp: Some(get_timestamp())
        };

        let ans = test_spool.get_length();
        assert_eq!(ans, 330);
    }

    #[test]
    #[should_panic]
    fn test_spool_length_none() {
        let mut test_spool = Spool {
            roll_id: Some(Uuid::new_v4()),
            roll_name: Some(String::from("crealtivity")),
            roll_weight: None,
            roll_length: None,
            timestamp: Some(get_timestamp())
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
            roll_weight: Some(1000),
            roll_length: Some(330),
            timestamp: Some(get_timestamp())
        };
       let rt = open_new_spool(&conn, &mut test_spool).unwrap();
       assert_eq!(rt, 1);
        let check_query = "SELECT * FROM spool";
        let exists_rt = conn.query_row(check_query, [], |row| Ok(Spool {
            roll_id: row.get(0)?,
            roll_name: row.get(1)?,
            roll_weight: row.get(2)?,
            roll_length: row.get(3)?,
            timestamp: row.get(4)?,
        })).unwrap();

        assert_eq!(exists_rt.roll_id.unwrap(), test_spool.roll_id.unwrap());
        assert_eq!(exists_rt.roll_name.unwrap(), "crealtivity".to_string());
        assert_eq!(exists_rt.roll_weight.unwrap(), 1000);
        assert_eq!(exists_rt.roll_length.unwrap(), 330);
        assert!(exists_rt.timestamp.unwrap() > 1734209754);

    }

}
