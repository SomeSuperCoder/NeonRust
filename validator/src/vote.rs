use base::{block::Block, ecdsa::{self, public_key_to_address, KeyPair}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Vote {
    pub block: Block,
    pub signature: Vec<u8>,
    pub pubkey: String
}

impl Vote {
    pub fn new(block: Block, keypair: KeyPair) -> Self {
        let signature = keypair.sign(&block.hash).unwrap();
        let signature = signature.to_bytes().as_slice().to_vec();
        
        Self {
            block,
            signature,
            pubkey: public_key_to_address(&keypair.public_key.to_sec1_bytes())
        }
    }

    pub fn agree(other: Vote, keypair: KeyPair) -> Self {
        Self::new(other.block, keypair)
    }
}
