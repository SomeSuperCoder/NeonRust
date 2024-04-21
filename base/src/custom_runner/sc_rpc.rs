use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct SCRPC {
    pub data: HashMap<String, String>
}
