use bs58::{decode, encode};use k256::{ecdsa::{signature::Signer, RecoveryId, Signature, SigningKey, }, schnorr::SignatureBytes, FieldBytes, Secp256k1, SecretKey};
use k256::{EncodedPoint, ecdsa::{VerifyingKey, signature::Verifier}};
use rand_core::OsRng;
use serde::de::Error;
use sha2::digest::generic_array::GenericArray;

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
}

pub fn public_key_to_address(public_key: &[u8]) -> String {
    encode(public_key).into_string()
}

pub fn address_to_public_key(address: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let some_vec = decode(address).into_vec()?;
    Ok(some_vec)
}

pub fn signature_to_bytes(signature: Signature) -> Vec<u8> {
    signature.to_bytes().as_slice().to_vec()
}

pub fn signature_from_bytes(bytes: Vec<u8>) -> Signature {
    Signature::from_bytes(&GenericArray::clone_from_slice(bytes.as_slice())).unwrap()
}
