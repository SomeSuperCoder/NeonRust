use crate::block::Block;
use crate::history::History;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn create_new_block(&self, seq: History) -> Block {
        Block{
            seq,
            latest_seq_hash: self.get_latest_hash()
        }
    }
    pub fn get_latest_hash(&self) -> String {
        let pev_elem = self.blocks.last();
        match pev_elem {
            Some(data) => data.seq.get_latest_hash(),
            None => String::from("0000000000000000000000000000000000000000000000000000000000000000")
        }
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
