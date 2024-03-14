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
            serde_json::to_string(&data).unwrap()
        );

        Self {
            data,
            hash
        }
    }

    pub fn valid_for(&self, blockchain: &Blockchain, cache: &Cache, slot_range: Vec<u128>) -> bool {
        if self.data.prev_block_hash != blockchain.get_latest_hash() {
            return false
        }
        if self.data.height != blockchain.get_latest_block_height() + 1 {
            return false
        }
        if !slot_range.contains(&self.data.slot) {
            return false
        }
        if !(self.data.slot > blockchain.get_latest_slot()) {
            return false
        }
        for tx in &self.data.seq {
            if !tx.valid_for(cache) {
                return false
            }
        }
        if self.hash != hasher::hash_string(
            serde_json::to_string(&self.data).unwrap()
        ) {
            return false;
        }

        true
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub struct BlockData {
    pub seq: Vec<Transaction>,
    pub prev_block_hash: String,
    pub height: u128,
    pub slot: u128
}
