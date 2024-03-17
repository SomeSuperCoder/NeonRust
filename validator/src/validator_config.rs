use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ValidatorConfig {
    seed_phrase: String,
    neighbours: Vec<String>,
    pull_from: String
}
