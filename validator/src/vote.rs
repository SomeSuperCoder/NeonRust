use base::{block::Block, ecdsa::{self, public_key_to_address, KeyPair}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Vote {
    pub block: Block,
    pub signature: Vec<u8>,
    pub pubkey: String
}

impl Vote {
    pub fn new(block: Block, keypair: &KeyPair) -> Self {
        let signature = keypair.sign(&block.hash).unwrap();
        let signature = ecdsa::signature_to_bytes(signature);
        
        Self {
            block,
            signature,
            pubkey: public_key_to_address(&keypair.public_key.to_sec1_bytes())
        }
    }

    pub fn agree(&self, keypair: &KeyPair) -> Self {
        Self::new(self.block.clone(), keypair)
    }

    pub fn verify_sginature(&self, keypair: &KeyPair) -> bool {
        let signature = ecdsa::signature_from_bytes(self.signature.clone());
        keypair.verify(&self.block.hash, signature.clone())
    }
}
