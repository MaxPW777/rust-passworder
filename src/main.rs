use serde::{Deserialize, Serialize};
use std::{
    env::args,
    io::{self, Write},
};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    hashed_master_password: String, // This should be a securely hashed version of the master password
}

#[derive(Serialize, Deserialize, Debug)]
struct Credential {
    service_name: String,
    encrypted_password: String, // The password should be encrypted
}

struct PasswordManager {
    credentials: Vec<Credential>,
}

struct EncryptionKey {
    key: Vec<u8>, // The actual key material, which should be securely generated and stored
}

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

fn new_password() {
    let mut service_name = String::new();
    let mut password = String::new();

    print!("Enter the name of the service: ");
    io::stdout().flush().unwrap(); // Flush stdout to ensure the prompt appears immediately
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut service_name).unwrap();

    // Trim the newline character at the end of the input
    service_name = service_name.trim_end().to_string();

    print!(
        "Enter the password for {} or type random for a random one: ",
        service_name
    );
    io::stdout().flush().unwrap(); // Flush stdout again
    stdin.read_line(&mut password).unwrap();

    // Trim the newline character at the end of the input
    password = password.trim_end().to_string();

    println!("Service: {}, Password: {}", service_name, password);
}

fn main() {
    let mut args = args();
    let mut _program = args.next().expect("program");

    if let Some(argument) = args.next() {
        match argument.as_str() {
            "add" => new_password(),
            "remove" => println!("removing password"),
            _ => intro(),
        }
    }
}
