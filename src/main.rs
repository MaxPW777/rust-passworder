use std::env::args;

fn show_help() {
    println!("RUST PASSWORD ENCRYPTOR");
    println!("-h : shows this screen");
    println!("-e : encrypts a word");
    println!("-d : decrypts a word");
}

fn encrypt_password(word: &str) -> String {
    String::from(word)
}
fn decrypt_password(word: &str, key: &str) -> String {
    String::from(word)
}

fn main() {
    let mut args = args();
    let mut _program = args.next().expect("program");
    if let Some(argument) = args.next() {
        match argument.as_str() {
            "-e" => {
                let word = args.next().expect("no input received");
                println!("{}", encrypt_password(&word));
            }
            "-d" => {
                let word = args.next().expect("no input received");
                let key = args.next().expect("no key received");
                println!("{}", decrypt_password(&word, &key));
            }
            _ => show_help(),
        }
    } else {
        show_help()
    }
}
