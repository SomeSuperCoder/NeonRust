use crate::account::AccountInfo;

#[derive(Debug, Default)]
pub struct Instruction {
    pub program_id: String, // Placeholder
    pub accounts: Vec<AccountInfo>,
    pub data: Vec<u8>
}
