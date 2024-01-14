use bs58::encode;
use k256::{ecdsa::{SigningKey, RecoveryId, Signature, signature::Signer}, FieldBytes, SecretKey};
use k256::{EncodedPoint, ecdsa::{VerifyingKey, signature::Verifier}};
use rand_core::OsRng;

pub struct KeyPair {
    pub private_key: Option<SigningKey>,
    pub public_key: VerifyingKey
}

impl KeyPair {
    pub fn sign(&self, message: &str) -> Result<Signature, &str> {
        match &self.private_key {
            Some(signing_key) => {
                let message = message.as_bytes();

                // Note: The signature type must be annotated or otherwise inferable as
                // `Signer` has many impls of the `Signer` trait (for both regular and
                // recoverable signature types).
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
