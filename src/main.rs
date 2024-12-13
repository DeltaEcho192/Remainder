use rusqlite::{Connection, Result};
use std::error::Error;

struct Spool {
    roll_id: Option<i32>,
    roll_name: Option<String>,
    roll_weight: Option<i32>,
    roll_length: Option<i32>,
    timestamp: Option<i32>,
}

struct Filament {}

fn main() {
    const conversion_factor: f32 = 3.0303;
    let db_path = "./3d_print.db";
    let db = Connection::open(db_path).unwrap();
    println!("Connection to database has been established");
    create_new_spool_tbl(&db);

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
                roll_id INTEGER PRIMARY KEY,
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

fn create_new_filament_tbl() {}

fn open_new_real(spool_info: Spool) {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Test_Spool_Creation() {
        let conn = Connection::open_in_memory().unwrap();
        create_new_spool_tbl(&conn).unwrap();
        create_new_spool_tbl(&conn).unwrap();
    }
}
