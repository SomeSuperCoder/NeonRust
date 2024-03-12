use sha2::{Sha256, Digest};

pub fn hash_string(string: String) -> String {
    hash_bytes(string.as_bytes())
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();

    hasher.update(bytes);
    let result = hasher.finalize();

    hex::encode(result)
}
