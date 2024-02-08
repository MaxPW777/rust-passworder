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
    pub fn add_credentials(&mut self, creds: Credential) {
        self.credentials.push(creds);
    }
}

impl fmt::Display for PasswordManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for credential in &self.credentials {
            writeln!(
                f,
                "Service Name: {}, Password: {}",
                credential.service_name, credential.encrypted_password
            )?;
        }
        Ok(())
    }
}
