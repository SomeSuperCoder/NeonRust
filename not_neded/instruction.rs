use crate::transaction::Transaction;
use crate::ecdsa::public_key_to_address;

#[derive(Clone)]
pub struct Instruction {
    pub sender: String,
    pub message_text: String,
    pub program_id: String
}

impl Instruction {
    pub fn from_tx(tx: &Transaction) -> Self {
        Self {
            message_text: tx.sender_part.message_text.clone(),
            program_id: tx.sender_part.program_id.clone(),
            sender: public_key_to_address(&tx.sender_part.public_key)
        }
    }
}
