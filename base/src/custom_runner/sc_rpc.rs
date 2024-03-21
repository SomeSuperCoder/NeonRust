use std::collections::HashMap;

use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SCRPC {
    pub data: HashMap<String, String>
}
