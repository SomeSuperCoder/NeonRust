use std::collections::HashMap;

#[derive(Debug)]
pub enum MessageData {
    Other(HashMap<String, MessageData>),
    Data(String)
}

pub struct Message {
    data: HashMap<String, MessageData>
}

impl Message {
    pub fn get(&self, key: &str) -> Option<&MessageData> {
        self.data.get(key)
    }
}
