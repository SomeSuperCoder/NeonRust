use std::thread::{self, JoinHandle};
use std::sync::Arc;

use crate::{
    cache::Cache, instruction::Instruction, native_runner::NativeRunner
};

pub struct InvokeHandler {
    pub cache: Cache,
    pub todo: Vec<Instruction>
}

impl InvokeHandler {
    pub fn invoke(mut self: Arc<Self>, instruction: Instruction) -> JoinHandle<()> {
        thread::spawn(|| {
            let lock = self.cache.lock(instruction.accounts);

            let result = NativeRunner::process_instrcution(instruction, Arc::clone(&self));

            self.cache.release(lock);

            match result {
                Ok(result) => {
                    result.changes.into_iter().map(|change| {self.cache.process_change(change)});
                },
                _ => {}
            }
        })
    }
}
