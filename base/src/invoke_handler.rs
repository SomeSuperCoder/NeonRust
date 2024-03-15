use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, RwLock};

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
    pub fn invoke(me: Arc<RwLock<Self>>, instruction: InstrcuctionSekelton) -> Option<JoinHandle<()>> {
        let potential_instrcution = me.read().unwrap().cache.form_instruction(instruction);
        if let Ok(instruction) = potential_instrcution {
            return Some(
                thread::spawn(move || {
                    let lock = me.write().unwrap().cache.lock(&instruction.accounts);
        
                    let result = NativeRunner::process_instrcution(
                        instruction, Arc::clone(&me)
                    );
        
                    me.write().unwrap().cache.release(lock);
        
                    match result {
                        Ok(result) => {
                            let _: Vec<_> = result.changes.into_iter().map(
                                |change| {
                                    me.write().unwrap().cache.process_change(change)
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
