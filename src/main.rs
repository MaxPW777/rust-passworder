use std::env::args;

fn show_help() {
    println!("RUST PASSWORD ENCRYPTOR");
    println!("-h : shows this screen");
    println!("-e : encrypts a word");
    println!("-d : decrypts a word");
}

fn main() {
    let mut args = args();
    let mut _program = args.next().expect("program");
    if let Some(argument) = args.next() {
        println!("{}", argument);
        match argument.as_str() {
            "-e" => println!("sussy baka"),
            "-d" => println!("sussD baka"),
            _ => show_help(),
        }
    }
}
