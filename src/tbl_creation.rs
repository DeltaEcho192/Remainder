use rusqlite::{Connection, Result};

pub fn create_new_spool_tbl(conn: &Connection) -> Result<(), &'static str> {
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

pub fn create_new_filament_tbl(conn: &Connection) -> Result<(), &'static str> {
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
