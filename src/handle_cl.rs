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
                if line.is_empty() {
                    continue;
                }

                let mut parts = line.splitn(3, ' '); 
                let raw_cmd = parts.next();

                let mut response = String::new();
                let command = raw_cmd.map(|c| c.to_uppercase()).unwrap_or_default();

                match command.as_str() {
                    "GET" => {
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
                    "SET" => {
                        let key = parts.next();
                        let value = parts.next(); 
                        if let (Some(key), Some(value)) = (key, value) {
                            db.set(key.to_string(), value.to_string());
                            response = "OK".to_string();
                        } else {
                            response = "Error: SET command requires a key and a value".to_string();
                        }
                    }
                    "DEL" => {
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
                    "KEYS" => {
                        let keys = db.keys();
                        response = format!("{:?}", keys);
                    }
                    "EXIT" => {
                        response = "Bye!".to_string();
                        let _ = stream.write_all(response.as_bytes());
                        let _ = stream.write_all(b"\n");
                        println!("Connection closed by client: {}", peer_addr);
                        break;
                    }
                    "HELP" => {
                        response = "Available commands:\n\
                                        GET <key>         - Retrieve the value of <key>\n\
                                        SET <key> <value> - Set or update the <key> with <value>\n\
                                        DEL <key>         - Delete the given <key>\n\
                                        KEYS              - List all keys\n\
                                        HELP              - Show this help message\n\
                                        EXIT              - Close the connection"
                                .to_string();

                    }
                    _ => {
                        response = format!("Error: Unknown command '{}'", line);
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
