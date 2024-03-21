use std::collections::HashMap;

use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Default)]
pub struct SCRPC {
    pub data: HashMap<String, String>
}
