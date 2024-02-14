use crate::instruction::{InstrcuctionSekelton, Instruction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub signatures: Vec<u8>,
    pub message: Message
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
pub struct Message {
    pub recent_block_hash: String,
    pub instructions: Vec<InstrcuctionSekelton>
}
