use std::thread::{self, JoinHandle};
use std::sync::{Arc, RwLock};

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
        println!("pre ins proc");
        if let Ok(instruction) = potential_instrcution {
            println!("ins proc 1");
            return Some(
                thread::spawn(move || {
                    println!("ins proc 2");
                    let mut to_lock = instruction.accounts.clone();
                    to_lock.push(instruction.program_account.clone());
                    println!("Before lock!");
                    let lock = me.write().unwrap().cache.lock(&to_lock);
                    println!("After lock!");
        
                    let result = NativeRunner::process_instrcution(
                        instruction, Arc::clone(&me)
                    );
        
                    me.write().unwrap().cache.release(lock);
        
                    match result {
                        Ok(result) => {
                            let _: Vec<_> = result.changes.into_iter().map(
                                |change| {
                                    println!("Processing change...");
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
 