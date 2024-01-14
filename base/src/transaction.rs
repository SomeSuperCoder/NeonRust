use crate::mutable_storage::MutableStorage;

#[derive(Clone, Debug, Default)]

pub struct Transaction {
    // TODO: implement std::fmt::Display
    pub sender_part: SenderPart,
    pub validator_part: ValidatorPart,
    pub storage: MutableStorage
}
#[derive(Clone, Debug, Default)]
pub struct SenderPart {
    pub program_id: String,
    pub message_text: String,
    pub public_key: Vec<u8>
}

#[derive(Clone, Debug, Default)]
pub struct ValidatorPart {
    pub public_key: Vec<u8>
}
