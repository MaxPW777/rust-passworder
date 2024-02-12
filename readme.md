# Rust Password Manager

This is a simple password manager written in Rust to securely store and manage your passwords.

## Features

- Generate strong, random passwords
- Encrypt and store passwords securely
- Retrieve passwords quickly
- Command-line interface for easy use

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your system. You can install them from [the official Rust website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone this repository:

```bash
git clone https://github.com/MaxPW777/rust-passworder
```

Navigate into the project directory:

```bash
cd rust-password-manager
```

Build the project:

```bash
cargo build --release
```

Run the password manager:

```bash
cargo run --release
```

### Usage

Adding a New Password

To add a new password, use the add command followed by the service name:

```bash
cargo run -- add "service_name"
```

You will be prompted to enter the password for the service. Optionally, you can also generate a random password.
Retrieving a Password

To retrieve a password, use the get command followed by the service name:

```bash
cargo run -- get "service_name"
```

The password will be displayed in the terminal.

### Listing All Services

To list all services with stored passwords, use the list command:

```bash
cargo run -- list
```

### Getting a single service

To get the password of a single service, use the get command:

```bash
cargo run -- get "service name"
```

### Deleting a service

To delete a service from the database, use the del command

```bash
cargo run -- del "service name"
```

You can also use the `-nuke` tag to delete all the passwords

### Security

This password manager uses AES encryption for storing passwords securely. Ensure you keep the master password safe and secure.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.
