use crate::{invoke_handler::InvokeHandler, transaction::Transaction};
use std::{sync::{Arc, Mutex}, thread::JoinHandle};
use std::collections::HashSet;

#[derive(Default)]
pub struct Runtime {
    pub invoke_handler: Arc<Mutex<InvokeHandler>>,
    locks: HashSet<u128>
}

impl Runtime {
    pub fn feed_tx_list(&self, tx_list: Vec<Transaction>) -> Vec<JoinHandle<()>> {
        println!("Feed list!");
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for tx in tx_list {
            let ih_access = Arc::clone(&self.invoke_handler);
            for signature in tx.signatures {
                ih_access.lock().unwrap().cache.spend(signature)
            }

            let instruction = tx.message.instruction;
            let join_handle = InvokeHandler::invoke(Arc::clone(&self.invoke_handler), instruction);
            if let Some(join_handle) = join_handle {
                handles.push(join_handle);
            }
        }

        handles
    }

    pub fn lock(&mut self) -> RuntimeLock {
        let id = (std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).expect("You time is crazy").as_millis()) as u128;
        self.locks.insert(id.clone());
        RuntimeLock {
            lock_id: id
        }
    }

    pub fn release(&mut self, lock: RuntimeLock) {
        self.locks.remove(&lock.lock_id);
    }
}

#[derive(Default)]
pub struct RuntimeLock {
    lock_id: u128
}
