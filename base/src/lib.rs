pub mod hasher;
pub mod transaction;
pub mod ecdsa;
pub mod has_address;
pub mod system_program;
pub mod utils;
pub mod history;
pub mod blockchain;
pub mod block;
pub mod account;
pub mod instruction;
pub mod native_runner;
pub mod program_result;
pub mod cache;
pub mod invoke_handler;
pub mod runtime;
pub mod timestamp;

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

    // #[test]
    // fn lock_test() {
    //     let mut cache = Cache::default();

    //     let test_account = Account {
    //         data: Vec::new(),
    //         pubkey: "aboba".to_string(),
    //         owner: "capybara".to_string(),
    //         atoms: 1_000_000,
    //         executable: false,
    //         latest_nonce: 0
    //     };

    //     let account_info = AccountInfo {
    //         underlying_account: test_account,
    //         is_signer: true,
    //         is_writable: true
    //     };

    //     let some_lock = cache.lock(&vec![account_info.clone()]);

    //     // if you remove this line, you will get into an infite loop
    //     cache.release(some_lock);
        
    //     cache.lock(&vec![account_info.clone()]);
    // }
}
