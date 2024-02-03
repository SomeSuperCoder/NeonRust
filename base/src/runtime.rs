use crate::{invoke_handler::InvokeHandler, transaction::Transaction};
use std::sync::{Arc, Mutex};

pub struct Runtime {
    invoke_handler: Arc<Mutex<InvokeHandler>>
}

impl Runtime {
    pub fn feed_tx_list(self: Arc<Self>, tx_list: Vec<Transaction>) {
        for tx in tx_list {
            for instruction in tx.message.instructions {
                // self.invoke_handler.lock().unwrap().invoke(instruction);
                InvokeHandler::invoke(Arc::clone(&self.invoke_handler), instruction);
            }
        }
    }
}
