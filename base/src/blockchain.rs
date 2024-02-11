use crate::block::{Block, BlockData};
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn create_new_block(&self, seq: Vec<Transaction>) -> Block {
        let data = BlockData {
            prev_block_hash: self.get_latest_hash(),
            seq: seq,
            height: self.get_latest_block_height() + 1
        };
        Block::from_data(data)
    }

    pub fn get_latest_hash(&self) -> String {
        let pev_elem = self.blocks.last();
        match pev_elem {
            Some(data) => data.hash.clone(),
            None => String::from("0000000000000000000000000000000000000000000000000000000000000000")
        }
    }

    pub fn get_latest_block_height(&self) -> u128 {
        (self.blocks.len() - 1) as u128
    }

    pub fn new() -> Blockchain {
        Blockchain{
            blocks: Vec::new()
        }
    }

    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }
}
