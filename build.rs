use std::{env, fs};

fn main() {
    let home_dir_var = match env::home_dir() {
        Some(mut path) => {
            println!("Your home directory, probably: {}", path.display());
            path.push(".remainder");
            path
        }
        None => {
            println!("Impossible to get your home dir!");
            panic!("Failed");
        }
    };
    match fs::create_dir(home_dir_var) {
        Ok(()) => println!("Successfully created data directory"),
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {
                println!("Program has already been installed on this device")
            }
            _ => panic!("There has been an unexpected err"),
        },
    };
}
