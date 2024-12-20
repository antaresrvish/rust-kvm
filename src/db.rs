use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::path::Path;

#[derive(Clone)]
pub struct DB {
    pub db: Arc<RwLock<HashMap<String, String>>>,
}

impl DB {
    pub fn new() -> DB {
        DB {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut db = self.db.write().expect("Failed to acquire write lock");
        db.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let db = self.db.read().expect("Failed to acquire read lock");
        db.get(key).cloned()
    }

    pub fn delete(&self, key: &str) -> bool {
        let mut db = self.db.write().expect("Failed to acquire write lock");
        db.remove(key).is_some()
    }

    pub fn keys(&self) -> Vec<String> {
        let db = self.db.read().expect("Failed to acquire read lock");
        db.keys().cloned().collect()
    }

    pub fn save_to_file(&self, file_path: &str) {
        let db = self.db.read().expect("Failed to acquire read lock");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("Failed to open file for writing");
        let mut writer = BufWriter::new(file);

        for (key, value) in db.iter() {
            writeln!(writer, "{}\t{}", key, value).expect("Failed to write to file");
        }
    }

    pub fn load_from_file(&self, file_path: &str) {
        if !Path::new(file_path).exists() {
            println!("Snapshot file not found: {}. Starting with an empty database.", file_path);
            return;
        }

        let file = File::open(file_path).expect("Failed to open file for reading");
        let reader = BufReader::new(file);
        let mut db = self.db.write().expect("Failed to acquire write lock");

        for line in reader.lines() {
            let line = line.expect("Failed to read line from file");
            if let Some((key, value)) = line.split_once('\t') {
                db.insert(key.to_string(), value.to_string());
            }
        }
        println!("Database loaded successfully from {}", file_path);
    }
}
