use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

use crate::{
    cache::Cache, instruction::Instruction, native_runner::NativeRunner
};

pub struct InvokeHandler {
    pub cache: Cache,
    pub todo: Vec<Instruction>
}

impl InvokeHandler {
    pub fn invoke(me: Arc<Mutex<Self>>, instruction: Instruction) -> JoinHandle<()> {
        thread::spawn(move || {
            let lock = me.lock().unwrap().cache.lock(&instruction.accounts);

            let result = NativeRunner::process_instrcution(
                instruction, Arc::clone(&me)
            );

            me.lock().unwrap().cache.release(lock);

            match result {
                Ok(result) => {
                    let _: Vec<_> = result.changes.into_iter().map(
                        |change| {
                            me.lock().unwrap().cache.process_change(change)
                        }
                    ).collect();
                },
                _ => {}
            }
        })
    }
}
