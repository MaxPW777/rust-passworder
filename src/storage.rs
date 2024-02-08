use std::{fs::File, io::BufReader, path::PathBuf};

use serde_json::Value;

use crate::models::{Credential, PasswordManager};

pub fn read_passwords(file_path: PathBuf) -> PasswordManager {
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

pub fn change_password() {}
