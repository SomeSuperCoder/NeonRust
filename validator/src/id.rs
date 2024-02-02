use base::ecdsa::KeyPair;
use k256::ecdsa::Signature;
use serde::Serialize;

struct Id {
    data: Vec<u8>,
    public_key: KeyPair,
    signtaure: Signature
}
