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
}
