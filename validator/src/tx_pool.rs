use base::transaction::Transaction;
use std::collections::HashSet;

pub struct TxPool {
    tx_list: HashSet<Transaction>
}

impl TxPool {
    pub fn new() -> Self {
        Self {
            tx_list: HashSet::new()
        }
    }

    pub fn verify_and_add(tx: Transaction) -> bool {
        todo!("Implement tx verifycation!")
    }

    pub fn get_nonce_for_user() {
        
    }
}
