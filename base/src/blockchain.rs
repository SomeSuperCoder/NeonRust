use crate::block::{Block, BlockData};
use crate::transaction::Transaction;

use std::fs;

#[derive(Debug)]
pub struct Blockchain {
    metadata: BlockchainMetaData
}

impl Blockchain {
    pub fn add_block(&mut self, block: Block) {
        self.metadata.latest_block_height = block.data.height.clone();
        self.metadata.latest_slot = block.data.slot.clone();
        self.metadata.latest_hash = block.hash.clone();
        
        fs::write(
            make_block_path(block.data.height),
            serde_json::to_string(&block).unwrap()).unwrap();
    }

    pub fn get_block(&self, height: u128) -> Option<Block> {
        Self::get_block_function(height)
    }

    pub fn get_block_function(height: u128) -> Option<Block> {
        match fs::read(make_block_path(height)) {
            Ok(block_json) => {
                serde_json::from_slice(block_json.as_slice()).unwrap()
            },
            _ => None
        }
    }

    pub fn create_new_block(&self, seq: Vec<Transaction>, slot: u128) -> Block {
        let data = BlockData {
            prev_block_hash: self.get_latest_hash(),
            seq: seq,
            height: self.get_latest_block_height() + 1,
            slot
        };
        Block::from_data(data)
    }

    pub fn get_latest_hash(&self) -> String {
        self.metadata.latest_hash.clone()
    }

    pub fn get_latest_block_height(&self) -> u128 {
        self.metadata.latest_block_height.clone()
    }

    pub fn get_latest_slot(&self) -> u128 {
        self.metadata.latest_slot.clone()
    }

    pub fn new() -> Blockchain {
        Blockchain {
            metadata: BlockchainMetaData::default()
        }
    }

    pub fn load() -> Self {
        let mut max_num: u128 = 0;

        for entry in fs::read_dir("./neon_validator/blockchain/").unwrap() {
            let entry = entry.unwrap();
            let num: u128 = entry.path().file_name().unwrap().to_str().unwrap().parse().unwrap();

            if num > max_num {
                max_num = num;
            }
        }

        let latest_block = Self::get_block_function(max_num.clone()).unwrap_or_else(|| {
            let mut block = Block::default();
            block.hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
            block
        });
        
        Self {
            metadata: BlockchainMetaData {
                latest_block_height: max_num,
                latest_hash: latest_block.hash,
                latest_slot: latest_block.data.slot
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct BlockchainMetaData {
    latest_hash: String,
    latest_block_height: u128,
    latest_slot: u128
}

pub fn make_block_path(height: u128) -> String {
    format!("./neon_validator/blockchain/{}", height)
}
