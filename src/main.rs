use std::{env::args, path::PathBuf, vec};

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
    args.next().expect("program");
    let filepath: PathBuf = PathBuf::from("passwords.json");
    let mut passwords = read_passwords(&filepath);

    if let Some(argument) = args.next() {
        match argument.as_str() {
            "add" => new_password(
                passwords,
                args.next().expect("no service name found"),
                &filepath,
            ),
            "remove" => remove_password(),
            "list" => println!("{}", passwords),
            _ => intro(),
        }
    }
}
