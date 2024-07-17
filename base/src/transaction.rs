use crate::instruction::InstrcuctionSekelton;
use serde::{Deserialize, Serialize};
use crate::cache::Cache;
use crate::ecdsa::{self, DoubleSignature, KeyPair};

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Hash, PartialEq, Eq)]
pub struct Transaction {
    pub signatures: Vec<Vec<u8>>,
    pub message: Message
}

impl Transaction {
    pub fn valid_for(&self, cache: &Cache) -> bool {
        for signature in &self.signatures {
            if cache.is_spent(signature.clone()) {
                println!("TX verify error: Sig dup");
                return false
            }
        }

        // Verify signatures
        let mut ok_count = 0;
        let mut signer_count = 0;

        for account in &self.message.instruction.accounts {
            if account.is_signer {
                signer_count += 1;
                let vk_obj = ecdsa::TriplePublicKey::from_address(account.pubkey.clone());
                println!("Here!");
                println!("Loading account: {}", account.pubkey.clone());
                if let Some(vk_obj) = vk_obj {
                    println!("Cond1!");
                    let keypair = KeyPair {
                        public_key: vk_obj.object,
                        private_key: None
                    };
                    for signature in &self.signatures {
                        let message_string = &serde_json::to_string(&self.message).unwrap();
                        println!("Checking message: {}", message_string);
                        let signature = DoubleSignature::from_bytes(signature.clone());
                        if let Some(signature) = signature {
                            println!("Cond2!");
                            if keypair.verify(message_string, signature.object) {
                                println!("Cond3!");
                                ok_count += 1;
                                break;
                            }
                        } else {
                            return false;
                        }
                    }
                } else {
                    println!("Error loading account");
                    return false;
                }
            }
        }

        ok_count == signer_count
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
#[derive(Hash, PartialEq, Eq)]
pub struct Message {
    pub nonce: String,
    pub instruction: InstrcuctionSekelton
}
