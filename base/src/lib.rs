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
    // fn test_msg() {
    //     let test = MessageData::Other(
    //         HashMap::from(
    //             [
    //                 (
    //                     "".to_string(),
    //                     MessageData::Data("ABOBA".to_string())
    //                 )
    //             ]
    //         )
    //     );

    //     println!("{:?}", test)
    // }

    // #[test]
    // fn test_runtime() {
    //     use wrapper::Wrapper;
    //     use blockchain::Blockchain;
    //     use cache::Cache;
    //     use std::sync::Arc;
    //     use second_runtime::Runtime;

    //     let wrapper = Wrapper {
    //         blockchain: Arc::new(Blockchain::new()),
    //         cache: Arc::new(Cache::default()),
    //         runtime: Runtime
    //     };

    //     fn some_func() {
    //         struct A {
    //             test: B,
    //         }
            
    //         struct B {
    //             parent: Arc<A>,
    //         }
            
    //         impl A {
    //             fn new() -> Self {
    //                 let parent = Arc::new(Self { test: B::new(Arc::clone(&parent)) });
    //                 Self { test: B::new(parent) }
    //             }
    //         }
            
    //         impl B {
    //             fn new() -> Self {
    //                 let parent: Arc<A>; // Declare the `parent` variable
            
    //                 // Initialize the `parent` variable within the `new` function
    //                 parent = Arc::new(A { test: B::new(Arc::clone(&parent)) });
            
    //                 Self { parent }
    //             }
    //         }
    //     }
    // }
}
