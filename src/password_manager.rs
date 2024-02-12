use std::{
    io::{self, Write},
    path::Path,
};

use crate::{
    models::{Credential, PasswordManager},
    storage::save_passwords,
    utils::generate_random_password,
};

fn password_input(service_name: String) -> Credential {
    let mut password = String::new();
    let stdin = io::stdin();
    // We get `Stdin` here.

    print!(
        "Enter the password for {} or type random for a random one: ",
        service_name
    );
    io::stdout().flush().unwrap();
    // Flush stdout again
    stdin.read_line(&mut password).unwrap();

    // Trim the newline character at the end of the input
    password = password.trim_end().to_string();

    // checks to see if user typed random
    if password.trim() == "random" {
        password = generate_random_password();
        println!("your new password is {}", password);
    }

    Credential {
        service_name,
        encrypted_password: password,
    }
}

pub fn new_password(mut password_manager: PasswordManager, service_name: String, filepath: &Path) {
    match password_manager.credential_exists(&service_name) {
        false => {
            let creds = password_input(service_name);
            password_manager.add_credentials(creds);
        }
        true => println!("password already exists"),
    }
    save_passwords(filepath, password_manager)
}

pub fn remove_password(
    mut password_manager: PasswordManager,
    service_name: String,
    filepath: &Path,
) {
    match password_manager.credential_exists(&service_name) {
        true => {
            match password_manager.delete_credentials(&service_name) {
                Err(error) => println!("an error has occured: {}", error),
                Ok(()) => println!("credential has been deleted"),
            }
            save_passwords(filepath, password_manager)
        }
        false => println!("password does not exist"),
    }
}

pub fn get_password(mut password_manager: PasswordManager, service_name: String) {
    match password_manager.get_credentials(&service_name) {
        Some(password) => println!("the password for {} is : {}", service_name, password),
        None => println!("password doesn't exist"),
    }
}
