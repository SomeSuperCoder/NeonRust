use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone)]
pub struct Account {
    pub data: Vec<u8>,
    pub pubkey: String,
    pub owner: String,
    pub atoms: u128,
    pub executable: bool,
    pub latest_nonce: u128
}

#[derive(Default, Debug, Clone)]
pub struct AccountInfo {
    pub underlying_account: Account,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone)]
pub struct AccountSkeleton {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool
}
