use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    env::args,
    fs::File,
    io::{self, BufReader, Write},
    path::PathBuf,
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

fn generate_random_password() -> String {
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    password
}

fn new_password(service_name: String) {
    let mut password = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.

    print!(
        "Enter the password for {} or type random for a random one: ",
        service_name
    );
    io::stdout().flush().unwrap(); // Flush stdout again
    stdin.read_line(&mut password).unwrap();

    // Trim the newline character at the end of the input
    password = password.trim_end().to_string();

    // checks to see if user typed random
    // match password{
    //     "random".to_owned() => rand::random(),
    //     _ =>
    // }

    println!("Service: {}, Password: {}", service_name, password);
}

fn remove_password() {
    let mut service_name = String::new();

    print!("Enter the name of the service: ");
    io::stdout().flush().unwrap(); // Flush stdout to ensure the prompt appears immediately
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut service_name).unwrap();
}

fn read_passwords(file_path: PathBuf) -> PasswordManager {
    let path: &str = file_path.to_str().unwrap();
    let file: File = File::open(path).expect("file not found");
    let reader: BufReader<File> = BufReader::new(file);

    let json: serde_json::Value =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");

    let creds: Vec<Credential> = json
        .get("credentials")
        .and_then(|credentials: &Value| serde_json::from_value(credentials.clone()).ok())
        .expect("not getting anything man");

    PasswordManager { credentials: creds }
}

fn main() {
    let mut args = args();
    let mut _program = args.next().expect("program");
    let path: PathBuf = PathBuf::from("src/passwords.json");
    println!("{}", generate_random_password());

    if let Some(argument) = args.next() {
        match argument.as_str() {
            "add" => new_password(args.next().expect("no service name found")),
            "remove" => remove_password(),
            _ => intro(),
        }
    }
}
