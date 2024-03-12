use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum SystemInstrusction {
    CreateAccount { pubkey: String },
    CloseAccount,
    Send { amount: u128 },
    SetAuthority { authority: u128 },
    SetAdmin { admin: bool },
    HelloWorld
}
