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

    pub fn list_ids(&self) {
        for (i, _) in &self.data {
            println!("{}", i);
        }
    }

    pub fn vote(&mut self, vote: Vote) -> bool {
        let block = vote.block;

        let id_ref = self.data.entry(block.data.height.clone()).or_insert(HashMap::new());
        let entry = id_ref.entry(block.hash.clone());
        let block_votes = entry.or_insert(BlockVotes::new(block));

        block_votes.vote(vote.pubkey)
    }

    pub fn result_for(&self, block_id: u128, _100_percent: u128) -> Option<Block> {
        // 10 / 20 = 0.5 => 50%
        let mut results: Vec<&Block> = Vec::new();

        for (id, block_voter_wrapper) in &self.data {
            if *id == block_id {
                for (_, block_voter) in block_voter_wrapper {
                    if (block_voter.count() / _100_percent) as f32 > config::REQUIRED_VOTE_PERCENT {
                        results.push(&block_voter.block)
                    }
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
        let min_height: u128 = blockchain.get_latest_block_height();
        let mut should_remove: Vec<u128> = Vec::new();

        for (height, _) in &self.data {
            if *height <= min_height {
                should_remove.push(height.clone());
            }
        }

        for remove in should_remove {
            self.data.remove(&remove);
        }
    }
}
