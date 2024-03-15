use crate::instruction::{InstrcuctionSekelton, Instruction};
use serde::{Deserialize, Serialize};
use crate::blockchain::Blockchain;
use crate::cache::Cache;
use crate::ecdsa::{self, KeyPair};

#[derive(Debug, Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub signatures: Vec<Vec<u8>>,
    pub message: Message
}

impl Transaction {
    pub fn valid_for(&self, cache: &Cache) -> bool {
        for signature in &self.signatures {
            if cache.is_spent(signature.clone()) {
                return false
            }
        }

        // Verify signatures
        let mut ok_count = 0;
        let mut signer_count = 0;

        for account in &self.message.instruction.accounts {
            if account.is_signer {
                signer_count += 1;
                if let Ok(vk_bytes) = ecdsa::address_to_public_key(account.pubkey.clone()) {
                    if let Ok(vk) = k256::ecdsa::VerifyingKey::from_sec1_bytes(&vk_bytes) {
                        let keypair = KeyPair {
                            public_key: vk,
                            private_key: None
                        };
                        for signature in &self.signatures {
                            let message_strig = &serde_json::to_string(&self.message).unwrap();
                            let signature = ecdsa::signature_from_bytes(signature.clone());
                            if keypair.verify(message_strig, signature) {
                                ok_count += 1;
                                break;
                            }
                        } 
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        ok_count == signer_count
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub struct Message {
    pub nonce: String,
    pub instruction: InstrcuctionSekelton
}
