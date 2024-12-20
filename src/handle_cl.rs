use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use crate::db::DB;

pub fn handle_client(mut stream: TcpStream, db: DB) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("New connection: {}", peer_addr);

    let mut reader = BufReader::new(stream.try_clone().unwrap());

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("Connection closed: {}", peer_addr);
                break;
            }
            Ok(_) => {
                let line = line.trim();
                let mut parts = line.split_whitespace();
                let cmd = parts.next();

                let mut response = String::new();

                match cmd {
                    Some("GET") => {
                        if let Some(key) = parts.next() {
                            if let Some(value) = db.get(key) {
                                response = value;
                            } else {
                                response = "(nil)".to_string();
                            }
                        } else {
                            response = "Error: GET command requires a key".to_string();
                        }
                    }
                    Some("SET") => {
                        let key = parts.next();
                        let value = parts.next();
                        if let (Some(key), Some(value)) = (key, value) {
                            db.set(key.to_string(), value.to_string());
                            response = "OK".to_string();
                        } else {
                            response = "Error: SET command requires a key and a value".to_string();
                        }
                    }
                    Some("DEL") => {
                        if let Some(key) = parts.next() {
                            let deleted = db.delete(key);
                            response = if deleted {
                                "1".to_string()
                            } else {
                                "0".to_string()
                            };
                        } else {
                            response = "Error: DEL command requires a key".to_string();
                        }
                    }
                    Some("KEYS") => {
                        let keys = db.keys();
                        response = format!("{:?}", keys);
                    }
                    Some("EXIT") => {
                        response = "Bye!".to_string();
                        let _ = stream.write_all(response.as_bytes());
                        let _ = stream.write_all(b"\n");
                        println!("Connection closed by client: {}", peer_addr);
                        break;
                    }
                    Some(unknown) => {
                        response = format!("Error: Unknown command '{}'", unknown);
                    }
                    None => {
                        continue;
                    }
                }

                response.push('\n');
                let _ = stream.write_all(response.as_bytes());
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", peer_addr, e);
                break;
            }
        }
    }
}
