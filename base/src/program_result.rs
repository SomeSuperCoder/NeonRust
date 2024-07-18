use crate::account::Account;

#[derive(Default)]
pub struct ProgramResult {
    pub changes: Vec<AccountChange>
}

impl ProgramResult {
    pub fn to_wrapped(&self, by: String) -> Vec<AccountChangeWrapper> {
        self.changes.clone().into_iter().map(|change| {
            AccountChangeWrapper {
                by: by.clone(),
                underlying_account_change: change
            } 
        }).collect()
    }
}

#[derive(Clone)]
pub enum AccountChange {
    SetData { of: String, data: Vec<u8> },
    SetAtoms { of: String, amount: u128 },
    SetExecutable { of: String, executable: bool },
    CreateAccount { account: Account },
    CloseAccount { pubkey: String },
    StartValidating { account: Account },
    StopValidating { account: Account }
}

pub struct AccountChangeWrapper {
    pub by: String,
    pub underlying_account_change: AccountChange
}
