use crate::{hasher, transaction::Transaction};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub data: BlockData,
    pub hash: String,
}

impl Block {
    pub fn from_data(data: BlockData) -> Self {
        let hash = hasher::hash_string(
            format!("{:?}", data) // TODO: replce this with borsh serialize
        );

        Self {
            data,
            hash
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
pub struct BlockData {
    pub seq: Vec<Transaction>,
    pub prev_block_hash: String,
    pub height: u128
}
