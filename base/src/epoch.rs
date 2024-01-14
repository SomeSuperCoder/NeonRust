use k256::ecdsa::VerifyingKey;

pub struct Epoch {
    public_keys: Vec<VerifyingKey>
}
