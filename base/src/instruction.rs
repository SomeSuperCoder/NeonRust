use crate::account::{AccountInfo, AccountSkeleton};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Instruction {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub data: Vec<u8>
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub struct InstrcuctionSekelton {
    pub program_id: String,
    pub accounts: Vec<AccountSkeleton>,
    pub data: Vec<u8>
}
