use std::collections::HashMap;
use std::error::Error;
use crate::transaction::Transaction;
use crate::system_program::SystemProgram;

pub trait ProgramTrait {
    fn will_touch(&self, tx: Transaction) -> Result<Vec<String>, Box<dyn Error>>;

    fn execute(&self, tx: Transaction);
}
