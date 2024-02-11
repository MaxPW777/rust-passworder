use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    hashed_master_password: String, // This should be a securely hashed version of the master password
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub service_name: String,
    pub encrypted_password: String, // The password should be encrypted
}

pub struct PasswordManager {
    pub credentials: Vec<Credential>,
}

// pub struct EncryptionKey {
//     key: Vec<u8>, // The actual key material, which should be securely generated and stored
// }

impl PasswordManager {
    pub fn credential_exists(&mut self, service_name: &String) -> bool {
        for credential in &self.credentials {
            // Iterate over a reference to avoid moving
            if &credential.service_name == service_name {
                return true;
            }
        }
        false
    }
    pub fn add_credentials(&mut self, creds: Credential) {
        self.credentials.push(creds); // Push creds after the loop
    }
    pub fn delete_credentials(&mut self, service_name: &str) -> Result<(), &str> {
        let initial_len = self.credentials.len();

        self.credentials
            .retain(|credential| credential.service_name != service_name);

        if self.credentials.len() == initial_len {
            // No credentials were removed, indicating the service name was not found
            Err("Service name not found")
        } else {
            Ok(())
        }
    }

    pub fn get_credentials(&mut self, service_name: &String) -> Option<&String> {
        for credential in &self.credentials {
            // Iterate over a reference to avoid moving
            if &credential.service_name == service_name {
                return Some(&credential.encrypted_password);
            }
        }
        None
    }
}

impl fmt::Display for PasswordManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.credentials.is_empty() {
            writeln!(f, "No credentials found")?;
        } else {
            for credential in &self.credentials {
                writeln!(
                    f,
                    "Service Name: {}, Password: {}",
                    credential.service_name, credential.encrypted_password
                )?;
            }
        }
        Ok(())
    }
}
