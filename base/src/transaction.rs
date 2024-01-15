use crate::mutable_storage::MutableStorage;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]

pub struct Transaction {
    // TODO: implement std::fmt::Display
    pub sender_part: SenderPart,
    pub validator_part: ValidatorPart,
    pub storage: MutableStorage
}

impl Transaction {
    pub fn get_self(&self) -> Transaction {
        return self.to_owned();
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SenderPart {
    pub program_id: String,
    pub message_text: String,
    pub public_key: Vec<u8>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ValidatorPart {
    pub public_key: Vec<u8>
}
