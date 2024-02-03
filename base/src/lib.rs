pub mod hasher;
pub mod transaction;
pub mod ecdsa;
pub mod has_address;
pub mod system_program;
pub mod utils;
// pub mod mutable_storage;
// pub mod program;
// pub mod account;
// pub mod wrapper;
pub mod history;
pub mod blockchain;
pub mod block;
pub mod epoch;
pub mod account;
pub mod instruction;
pub mod native_runner;
pub mod program_result;
// pub mod system_program;
// pub mod program_runtime;
pub mod cache;
pub mod invoke_handler;
pub mod runtime;
// pub mod blockchain_wrapper;
// pub mod process_order;
// pub mod executor;
// pub mod instruction;
// pub mod second_runtime;
// pub mod message;

#[cfg(test)]
mod tests {
    use self::{account::{Account, AccountInfo}, cache::Cache};

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
    
    #[test]
    fn test_clone() {
        let a = "asdasda".to_string();
        let b = a.clone();

        println!("{}, {}", a, b)
    }

    #[test]
    fn lock_test() {
        let mut cache = Cache::default();

        let test_account = Account {
            data: Vec::new(),
            pubkey: "aboba".to_string(),
            owner: "capybara".to_string(),
            atoms: 1_000_000,
            executable: false
        };

        let account_info = AccountInfo {
            underlying_account: test_account,
            is_signer: true,
            is_writable: true
        };

        let some_lock = cache.lock(&vec![account_info.clone()]);

        cache.release(some_lock);
        cache.lock(&vec![account_info.clone()]);
    }
}
