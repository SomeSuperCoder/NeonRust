use crate::{invoke_handler::InvokeHandler, transaction::Transaction};
use std::sync::Arc;

pub struct Runtime {
    invoke_handler: Arc<InvokeHandler>
}

impl Runtime {
    pub fn feed_tx_list(&mut self, tx_list: Vec<Transaction>) {
        for tx in tx_list {
            for instruction in tx.message.instructions {
                self.invoke_handler.invoke(instruction);
            }
        }
    }
}
