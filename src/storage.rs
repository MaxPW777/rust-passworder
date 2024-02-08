use std::{fs::File, io::BufReader, path::Path};

use serde_json::{json, Value};

use crate::models::{Credential, PasswordManager};
use serde_json::to_writer;

pub fn read_passwords(file_path: &Path) -> PasswordManager {
    let path: &str = file_path.to_str().unwrap();
    let file: File = File::open(path).expect("file not found");
    let reader: BufReader<File> = BufReader::new(file);

    let json: serde_json::Value =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");

    // Represents a vector of credentials.
    let creds: Vec<Credential> = json
        .get("credentials")
        .and_then(|credentials: &Value| serde_json::from_value(credentials.clone()).ok())
        .expect("could not get credentials from file");

    PasswordManager { credentials: creds }
}

pub fn save_passwords(file_path: &Path, passwords: PasswordManager) {
    let path: &str = file_path.to_str().unwrap();
    let mut file: File = File::create(path).expect("file not found");
    to_writer(&mut file, &json!({"credentials" : &passwords.credentials}))
        .expect("failed to write passwords to file");
}
