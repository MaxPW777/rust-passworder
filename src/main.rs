use std::{env::args, path::PathBuf};

use password_manager::{new_password, remove_password};
use storage::read_passwords;

mod models;
mod password_manager;
mod storage;
mod utils;

fn intro() {
    println!(
        "
RUST PASSWORD MANAGER
USAGE: 
    list                list passwords
    add 'service name'  add password
    get 'service name'  gets password
    help                shows this screen
    quit                quits program
    "
    )
}

fn main() {
    let mut args = args();
    let mut _program = args.next().expect("program");
    let path: PathBuf = PathBuf::from("passwords.json");

    let passwords = read_passwords(path);

    if let Some(argument) = args.next() {
        match argument.as_str() {
            "add" => new_password(args.next().expect("no service name found")),
            "remove" => remove_password(),
            _ => intro(),
        }
    }
}
