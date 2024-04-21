use base::{block::Block, ecdsa::{self, KeyPair, TriplePublicKey}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Vote {
    pub block: Block,
    pub signature: Vec<u8>,
    pub pubkey: String
}

impl Vote {
    pub fn new(block: Block, keypair: &KeyPair) -> Self {
        let pubkey = TriplePublicKey::from_object(keypair.public_key).unwrap().address;
        let signature = keypair.sign(&block.hash).unwrap();
        let signature = ecdsa::signature_to_bytes(signature);
        
        Self {
            block,
            signature,
            pubkey
        }
    }

    pub fn agree(&self, keypair: &KeyPair) -> Self {
        Self::new(self.block.clone(), keypair)
    }

    pub fn verify_sginature(&self, keypair: &KeyPair) -> bool {
        let signature = ecdsa::signature_from_bytes(self.signature.clone());
        if let Some(signature) = signature {
            keypair.verify(&self.block.hash, signature.clone())
        } else {
            false
        }
    }
}
