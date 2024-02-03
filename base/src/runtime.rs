use crate::{invoke_handler::InvokeHandler, transaction::Transaction};
use std::{sync::{Arc, Mutex}, thread::JoinHandle};

pub struct Runtime {
    invoke_handler: Arc<Mutex<InvokeHandler>>
}

impl Runtime {
    pub fn feed_tx_list(self: Arc<Self>, tx_list: Vec<Transaction>) -> Vec<JoinHandle<()>> {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for tx in tx_list {
            for instruction in tx.message.instructions {
                let join_handle = InvokeHandler::invoke(Arc::clone(&self.invoke_handler), instruction);
                handles.push(join_handle);
            }
        }

        handles
    }
}
