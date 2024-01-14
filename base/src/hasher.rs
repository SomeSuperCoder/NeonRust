use sha2::{Sha256, Digest};

pub fn hash_string(string: String) -> String {
    // Create a SHA256 hasher.
    let mut hasher = Sha256::new();

    // Write input message
    hasher.update(string.as_bytes());

    // Get the hash result.
    let result = hasher.finalize();

    // Convert the hash result to a hex string.
    let hex_result = hex::encode(result);

    hex_result
}
