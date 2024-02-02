use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Cache {
    data: HashMap<String, String>
}

impl Cache {
    pub fn cache(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.data.get(&key)
    }
}
