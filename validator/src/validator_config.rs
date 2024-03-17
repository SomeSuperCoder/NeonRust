use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ValidatorConfig {
    pub seed_phrase: String,
    pub neighbours: Vec<String>,
    pub pull_from: String
}
