use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum SystemInstrusction {
    CreateAccount { pubkey: String },
    CloseAccount,
    Send { amount: u128 },
    SetAuthority { authority: u128 },
    SetAdmin { admin: bool },
    HelloWorld,
    CreateSystemAccount { pubkey: String },
}
