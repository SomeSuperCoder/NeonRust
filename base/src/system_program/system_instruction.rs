use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum SystemInstrusction {
    CreateAccount,
    CloseAccount,
    Send { amount: u128, receiver_account_create: bool },
    Mint { amount: u128, receiver_account_create: bool }
}
