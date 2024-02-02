use crate::transaction::Transaction;

#[derive(Debug, Default)]
pub struct Block {
    pub seq: Vec<Transaction>
}
