use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
}