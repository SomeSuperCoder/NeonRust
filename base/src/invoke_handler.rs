use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

use crate::instruction::InstrcuctionSekelton;
use crate::{
    cache::Cache, instruction::Instruction, native_runner::NativeRunner
};

#[derive(Default)]
pub struct InvokeHandler {
    pub cache: Cache,
    pub todo: Vec<Instruction>
}

impl InvokeHandler {
    pub fn invoke(me: Arc<Mutex<Self>>, instruction: InstrcuctionSekelton) -> Option<JoinHandle<()>> {
        let potential_instrcution = me.lock().unwrap().cache.form_instruction(instruction);
        if let Ok(instruction) = potential_instrcution {
            return Some(
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
                    };
                })
            );
        } else {
            None
        }
    }
}
