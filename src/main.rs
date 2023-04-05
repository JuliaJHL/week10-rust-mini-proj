use sled::{Config, Db};
use std::io::{self, Write};

fn store_password(db: &Db) {
    print!("Enter service name: ");
    io::stdout().flush().unwrap();
    let mut service = String::new();
    io::stdin().read_line(&mut service).unwrap();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    let encrypted_password = password.trim().to_string();

    match db.insert(service.trim().as_bytes(), encrypted_password.as_bytes()) {
        Ok(_) => println!("Password stored successfully!"),
        Err(e) => println!("Failed to store password: {}", e),
    }
}

fn retrieve_password(db: &Db) {
    print!("Enter service name: ");
    io::stdout().flush().unwrap();
    let mut service = String::new();
    io::stdin().read_line(&mut service).unwrap();

    match db.get(service.trim().as_bytes()) {
        Ok(Some(password_bytes)) => {
            let password = String::from_utf8_lossy(&password_bytes);
            println!("Password for {} is: {}", service.trim(), password);
        }
        Ok(None) => println!("No password found for {}", service.trim()),
        Err(e) => println!("Failed to retrieve password: {}", e),
    }
}

fn main() {
    let db = match Config::default().path("passwords.sled").open() {
        Ok(db) => db,
        Err(e) => panic!("Failed to open database: {}", e),
    };

    loop {
        print!("Enter command (store/retrieve/quit): ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        match command.trim() {
            "store" => store_password(&db),
            "retrieve" => retrieve_password(&db),
            "quit" => break,
            _ => println!("Invalid command"),
        }
    }
}
