use crate::print_add::get_current_spool;
use crate::print_structs::*;
use rusqlite::Connection;

pub fn lifetime_statistics(conn: &Connection) -> (f32, f32, i32) {
    let lifetime_query =
        "SELECT SUM(print_weight), SUM(print_length), SUM(print_time) FROM filament";

    let lifetime_rt = conn
        .query_row(lifetime_query, [], |row| {
            Ok(Filament {
                print_id: None,
                print_weight: row.get(0)?,
                print_length: row.get(1)?,
                print_time: row.get(2)?,
                roll_id: None,
            })
        })
        .unwrap();
    let lifetime_output = (
        lifetime_rt.print_weight.unwrap_or_default(),
        lifetime_rt.print_length.unwrap_or_default(),
        lifetime_rt.print_time.unwrap_or_default(),
    );
    lifetime_output
}

pub fn check_remaining(conn: &Connection) -> (f32, f32) {
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

    let remaining_length = original_rt.roll_length.unwrap() - accu_rt.print_length.unwrap_or_default();
    let remaining_weight = original_rt.roll_weight.unwrap() - accu_rt.print_weight.unwrap_or_default();
    let remaining_rt = (remaining_weight, remaining_length);
    remaining_rt
}
