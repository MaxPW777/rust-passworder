use std::io::{self, Write};

use crate::utils::generate_random_password;

fn password_input(service_name: String) {
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

    println!("Service: {}, Password: {}", service_name, password);
}

pub fn new_password(service_name: String) {
    password_input(service_name);
}

pub fn remove_password() {
    let mut service_name = String::new();

    print!("Enter the name of the service: ");
    io::stdout().flush().unwrap(); // Flush stdout to ensure the prompt appears immediately
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut service_name).unwrap();
}
