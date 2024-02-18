use std::collections::HashMap;
use base::block::Block;
use base::blockchain::Blockchain;

use crate::{block_votes::BlockVotes, vote::Vote};

pub struct BlockVoter {
    data: HashMap<u128, HashMap<String, BlockVotes>>
}

impl BlockVoter {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    // this will return if we voted for the block, or it already exists
    pub fn vote(&mut self, vote: Vote) -> bool {
        let return_value: bool;
        let block = vote.block;

        let id_ref = self.data.entry(block.data.height.clone()).or_insert(HashMap::new());
        let entry = id_ref.entry(block.hash.clone());

        match entry {
            std::collections::hash_map::Entry::Occupied(_) => {
                return_value = true;
            },
            std::collections::hash_map::Entry::Vacant(_) => {
                return_value = false;
            }
        }

        let block_votes = entry.or_insert(BlockVotes::new(block));
        block_votes.vote(vote.pubkey);

        return_value
    }

    pub fn result_for(&self, block_id: u128, _100_percent: u128) -> Option<Block> {
        // 10 / 20 = 0.5 => 50%
        let mut results: Vec<&Block> = Vec::new();

        for (id, block_voter_wrapper) in &self.data {
            for (hash, block_voter) in block_voter_wrapper {
                if (block_voter.count() / _100_percent) as f32 > config::REQUIRED_VOTE_PERCENT {
                    results.push(&block_voter.block)
                }
            }
        }

        match results.len() {
            1 => {
                Some((*results[0]).clone())
            },
            _ => {
                None
            }
        }
    }

    pub fn filter(&mut self, blockchain: &Blockchain) {
        todo!();
    }
}
