use crate::instruction::Instruction;

#[derive(Debug, Default)]
pub struct Transaction {
    pub signatures: Vec<u8>,
    pub message: Message
}

#[derive(Debug, Default)]
pub struct Message {
    pub recent_block_hash: String,
    pub instructions: Vec<Instruction>
}
