#[derive(Default, Debug, Clone)]
pub struct Account {
    pub data: Vec<u8>,
    pub pubkey: String, // Placeholder!!!
    pub owner: String, // Placeholder!!!
    pub atoms: u128,
    pub executable: bool
}

#[derive(Default, Debug, Clone)]
pub struct AccountInfo {
    pub underlying_account: Account,
    pub is_signer: bool,
    pub is_writable: bool,
}
