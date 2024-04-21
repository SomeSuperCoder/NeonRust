use std::str::FromStr;
use bs58::{decode, encode};
use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use k256::ecdsa::{VerifyingKey, signature::Verifier};
use k256::pkcs8::der::Encode;
use k256::pkcs8::{DecodePublicKey, EncodePublicKey};
use k256::EncodedPoint;
use rand::RngCore;
use rand_core::OsRng;
use bip39::{self, Mnemonic};

#[derive(Clone)]
pub struct KeyPair {
    pub private_key: Option<SigningKey>,
    pub public_key: VerifyingKey
}

impl KeyPair {
    pub fn sign(&self, message: &str) -> Result<Signature, &str> {
        match &self.private_key {
            Some(signing_key) => {
                let message = message.as_bytes();
                
                let signature: Signature = signing_key.sign(message);

                Ok(signature)
            }
            None => {
                Err(
                    "No private key in keypair"
                )
            }
        }
    }

    pub fn verify(&self, message: &str, signature: Signature) -> bool {
        self.public_key.verify(message.as_bytes(), &signature).is_ok()
    }

    pub fn random() -> KeyPair { 
        let private_key = SigningKey::random(&mut OsRng);
        KeyPair {
            public_key: VerifyingKey::from(&private_key),
            private_key: Some(private_key)
        }
    }

    pub fn recover(seed_phrase: String) -> Option<KeyPair> {
        if let Ok(mnemonic) = Mnemonic::from_str(&seed_phrase) {
            let private_key = SigningKey::from_slice(&mnemonic.to_seed("")[0..24]).unwrap();
            Some(
                KeyPair {
                    public_key: VerifyingKey::from(&private_key),
                    private_key: Some(private_key)
                }
            )
        } else {
            None
        }
    }
}


pub fn generate_seed_phrase() -> String {
    let mut random_bytes: [u8; 16] = [1; 16];
    rand::thread_rng().fill_bytes(&mut random_bytes);
    let menmonic = Mnemonic::from_entropy_in(bip39::Language::English, &random_bytes).unwrap();
    let seed_phrase = menmonic.to_string();

    seed_phrase
}

pub fn public_key_to_address(public_key: &[u8]) -> String {
    encode(public_key).into_string()
}

pub fn address_to_public_key(address: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let some_vec = decode(address).into_vec()?;
    Ok(some_vec)
}

pub fn signature_to_bytes(signature: Signature) -> Vec<u8> {
    signature.to_der().to_der().unwrap().as_slice().to_vec()
}

pub fn signature_from_bytes(bytes: Vec<u8>) -> Option<Signature> {
    if let Ok(a) = Signature::from_der(bytes.as_slice()) {
        Some(a)
    } else {
        None
    }
}

pub struct TriplePublicKey {
    pub object: VerifyingKey,
    pub bytes: Vec<u8>,
    pub address: String
}

impl TriplePublicKey {
    pub fn from_bytes(source: Vec<u8>) -> Option<Self> {
        if let Ok(encoded_point) = &EncodedPoint::from_bytes(&source) {
            if let Ok(object) = VerifyingKey::from_encoded_point(encoded_point) {
                Some (
                    Self {
                        object,
                        bytes: source.clone(),
                        address: public_key_to_address(&source)
                    }
                )
            } else {
                None
            } 
        } else {
            None
        }
    }

    pub fn from_address(source: String) -> Option<Self> {
        if let Ok(source) = address_to_public_key(source) {
            Self::from_bytes(source)
        } else {
            None
        }
    }

    pub fn from_object(object: VerifyingKey) -> Option<Self> {
        Self::from_bytes(object.to_encoded_point(true).to_bytes().to_vec())
    }
}

pub struct DoubleSignature {
    pub object: Signature,
    pub bytes: Vec<u8>
}

impl DoubleSignature {
    pub fn from_bytes(source: Vec<u8>) -> Option<Self> {
        if let Ok(signature) = Signature::from_der(&source) {
            Some (
                Self {
                    object: signature,
                    bytes: source
                }
            )
        } else {
            None
        }
    }
}
