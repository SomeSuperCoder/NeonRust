use crate::{invoke_handler::InvokeHandler, transaction::Transaction};
use std::{sync::{Arc, Mutex, RwLock}, thread::JoinHandle};
use std::collections::HashSet;

#[derive(Default)]
pub struct Runtime {
    pub invoke_handler: Arc<RwLock<InvokeHandler>>
}

impl Runtime {
    pub fn feed_tx_list(&self, tx_list: Vec<Transaction>) -> Vec<JoinHandle<()>> {
        println!("Feed list!");
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for tx in tx_list {
            let ih_access = self.invoke_handler.read().unwrap();
            for signature in tx.signatures {
                ih_access.cache.spend(signature)
            }

            let instruction = tx.message.instruction;
            let join_handle = InvokeHandler::invoke(Arc::clone(&self.invoke_handler), instruction);
            if let Some(join_handle) = join_handle {
                handles.push(join_handle);
            }
        }

        handles
    }
}
