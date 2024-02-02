use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MutableStorage {
    data: HashMap<String, String>
}

impl MutableStorage {
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<&String> {
        self.data.get(&key)
    }
}
