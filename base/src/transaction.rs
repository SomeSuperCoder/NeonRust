use crate::instruction::{InstrcuctionSekelton, Instruction};
use serde::{Deserialize, Serialize};
use crate::blockchain::Blockchain;
use crate::cache::Cache;

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub signatures: Vec<u8>,
    pub message: Message
}

impl Transaction {
    pub fn validate(&self, blockchain: &Blockchain, cache: &Cache) -> bool {
        todo!();
    }

    pub fn valid_for(&self, blockchain: &Blockchain, cache: &Cache) -> bool {
        todo!();
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub struct Message {
    pub nonce: u128,
    pub instructions: Vec<InstrcuctionSekelton>
}
