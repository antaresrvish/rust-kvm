use rust_kvm::db::DB;
use rust_kvm::handle_cl::handle_client;
use std::env;
use std::net::TcpListener;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut port = "4578".to_string();

    if args.len() > 2 && args[1] == "--port" {
        port = args[2].clone();
    }

    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address).expect("Could not bind to port");
    println!("Server running on port {}", port);

    let db = DB::new();
    let snapshot_file = "database.kvm";

    db.load_from_file(snapshot_file);
    println!("Database loaded from {}", snapshot_file);

    let db_clone = db.clone();
    thread::spawn(move || {
        loop {
            db_clone.save_to_file(snapshot_file);
            println!("Database snapshot saved to {}", snapshot_file);
            std::thread::sleep(std::time::Duration::from_secs(120)); 
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_clone = db.clone();
                thread::spawn(move || {
                    handle_client(stream, db_clone);
                });
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    db.save_to_file(snapshot_file);
    println!("Database saved to {}", snapshot_file);
}

