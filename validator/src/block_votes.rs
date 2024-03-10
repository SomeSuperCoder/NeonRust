use base::block::Block;
use std::collections::HashSet;

pub struct BlockVotes {
    pub block: Block,
    pub votes: HashSet<String> // list of addresses
}

impl BlockVotes {
    pub fn new(block: Block) -> Self {
        Self {
            block,
            votes: HashSet::new()
        }
    }

    pub fn vote(&mut self, voter: String) -> bool {
        let did_instert = self.votes.insert(voter);

        if did_instert {
            true
        } else {
            false
        }
    }

    pub fn count(&self) -> u128 {
        self.votes.len() as u128
    }
}
