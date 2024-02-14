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

    pub fn vote(&mut self, voter: String) -> DidActuallyInsert {
        let did_instert = self.votes.insert(voter);

        if did_instert {
            DidActuallyInsert::Yes
        } else {
            DidActuallyInsert::No
        }
    }
}

pub enum DidActuallyInsert {
    Yes,
    No
}
