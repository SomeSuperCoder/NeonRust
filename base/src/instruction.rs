use crate::account::{AccountInfo, AccountSkeleton};
use crate::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Instruction {
    pub accounts: Vec<AccountInfo>,
    pub data: Vec<u8>,
    pub program_account: Account
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
#[derive(Hash, PartialEq, Eq)]
pub struct InstrcuctionSekelton {
    pub program_id: String,
    pub accounts: Vec<AccountSkeleton>,
    pub data: Vec<u8>
}
