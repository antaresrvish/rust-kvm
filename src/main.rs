use rust_kvm::db::DB;
use std::io::{self, Write};

fn main() {
    let db = DB::new();

    loop {
        print!("\x1B[2J\x1B[1;1H"); 
        io::stdout().flush().unwrap();

        println!("== Command Menu ==");
        println!("1) List all keys");
        println!("2) Get a key's value");
        println!("3) Create or update a key");
        println!("4) Delete a key");
        println!("5) Exit");
        print!("Your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                let keys = db.keys();
                println!("All keys: {:?}", keys);
            }
            "2" => {
                print!("Enter the key you want to retrieve: ");
                io::stdout().flush().unwrap();
                let mut key = String::new();
                io::stdin().read_line(&mut key).expect("Failed to read input");
                let key = key.trim();
                if let Some(value) = db.get(key) {
                    println!("Value for '{}' is: {}", key, value);
                } else {
                    println!("Key '{}' not found.", key);
                }
            }
            "3" => {
                print!("Enter the key you want to create/update: ");
                io::stdout().flush().unwrap();
                let mut key = String::new();
                io::stdin().read_line(&mut key).expect("Failed to read input");
                let key = key.trim().to_string();

                print!("Enter the value for '{}': ", key);
                io::stdout().flush().unwrap();
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("Failed to read input");
                let value = value.trim().to_string();

                db.set(key.clone(), value);
                println!("Key '{}' has been set/updated.", key);
            }
            "4" => {
                print!("Enter the key you want to delete: ");
                io::stdout().flush().unwrap();
                let mut key = String::new();
                io::stdin().read_line(&mut key).expect("Failed to read input");
                let key = key.trim();
                let deleted = db.delete(key);
                if deleted {
                    println!("Key '{}' deleted.", key);
                } else {
                    println!("Key '{}' not found.", key);
                }
            }
            "5" => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }

        println!("\nPress ENTER to continue...");
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy).expect("Failed to read input");
    }
}
