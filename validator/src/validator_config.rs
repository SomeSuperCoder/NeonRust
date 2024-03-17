use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ValidatorConfig {
    pub seed_phrase: String,
    pub neighbours: Vec<String>,
    pub pull_from: String
}

impl ValidatorConfig {
    pub fn load() -> Self {
        serde_json::from_str(fs::read_to_string("./neon_validator/config.json").unwrap().as_str()).unwrap()
    }
}
