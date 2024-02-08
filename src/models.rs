use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    hashed_master_password: String, // This should be a securely hashed version of the master password
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    service_name: String,
    encrypted_password: String, // The password should be encrypted
}

pub struct PasswordManager {
    pub credentials: Vec<Credential>,
}

pub struct EncryptionKey {
    key: Vec<u8>, // The actual key material, which should be securely generated and stored
}
