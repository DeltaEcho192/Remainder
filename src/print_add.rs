use rusqlite::{Connection, Result};
use uuid::Uuid;
use crate::print_structs::*;

pub struct RollId {
    pub roll_id: Uuid,
}

pub fn get_current_spool(conn: &Connection) -> Result<RollId> {
    let check_query =
        "SELECT r.roll_id FROM spool r WHERE r.roll_timestamp = (SELECT MAX(roll_timestamp) FROM spool)";

    let exists_rt = conn.query_row(check_query, [], |row| {
        Ok(RollId {
            roll_id: row.get(0).unwrap(),
        })
    });
    exists_rt
}

pub fn add_new_print(conn: &Connection, print: &mut Filament) -> Result<usize> {
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

pub fn open_new_spool(conn: &Connection, spool_info: &mut Spool) -> Result<usize> {
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
