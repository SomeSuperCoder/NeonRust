use crate::{hasher, transaction::Transaction};
use serde::{Serialize, Deserialize};
use crate::blockchain::Blockchain;
use crate::cache::Cache;

#[derive(Debug, Default, Clone)]
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

    pub fn valid_for(&self, blockchain: &Blockchain, cache: &Cache) {
        
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub struct BlockData {
    pub seq: Vec<Transaction>,
    pub prev_block_hash: String,
    pub height: u128
}
