#[derive(Default)]
pub struct ProgramResult {
    pub changes: Vec<AccountChange>
}

impl ProgramResult {
    pub fn set_atoms(&mut self, of: String, amount: u128) {
        self.changes.push(
            AccountChange::SetAtoms {
                of,
                amount
            }
        );
    }

    pub fn to_wrapped(&self, by: String) -> Vec<AccountChangeWrapper> {
        self.changes.into_iter().map(|change| {
            AccountChangeWrapper {
                by,
                underlying_account_change: change
            } 
        }).collect()
    }
}

pub enum AccountChange {
    SetData { of: String, data: Vec<u8> },
    SetAtoms { of: String, amount: u128 }
}

pub struct AccountChangeWrapper {
    pub by: String,
    pub underlying_account_change: AccountChange
}
