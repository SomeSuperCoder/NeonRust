use crate::account::{AccountInfo, AccountSkeleton};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Instruction {
    pub accounts: Vec<AccountInfo>,
    pub data: String,
    pub program_account: AccountInfo
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
#[derive(Hash, PartialEq, Eq)]
pub struct InstrcuctionSekelton {
    pub program_id: String,
    pub accounts: Vec<AccountSkeleton>,
    pub data: String
}
