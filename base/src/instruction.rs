use crate::account::{AccountInfo, AccountSkeleton};

#[derive(Debug, Default)]
pub struct Instruction {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub data: Vec<u8>
}

#[derive(Debug, Default)]
pub struct InstrcuctionSekelton {
    pub program_id: String,
    pub accounts: Vec<AccountSkeleton>,
    pub data: Vec<u8>
}
