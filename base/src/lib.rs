pub mod hasher;
pub mod transaction;
pub mod ecdsa;
pub mod has_address;
pub mod mutable_storage;
pub mod program;
pub mod account;
pub mod wrapper;
pub mod history;
pub mod blockchain;
pub mod block;
pub mod epoch;
pub mod system_program;
pub mod program_runtime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecdsa_test() {
        let message = "hello";
        let key_pair = ecdsa::KeyPair::random();
        let signature = match key_pair.sign(message) {
            Ok(value) => value,
            Err(e) => panic!()
        };
        println!("{:?}", key_pair.private_key.clone().unwrap().to_bytes());
        assert!(
           key_pair.verify(message, signature)
        )
    }
}
