use std::error::Error;
use std::sync::{Arc, Mutex};
use crate::cache::Cache;
use crate::instruction::Instruction;
use crate::system_program::SystemProgram;
use crate::mutable_storage::MutableStorage;
use crate::second_runtime::ProgramRequire;
use crate::second_runtime::InvokeHandler;

pub trait ProgramTrait {
    fn will_read(&self, instruction: &Instruction, cache: &Arc<Cache>) -> Result<ProgramRequire, Box<dyn Error>>;
    fn will_write(&self, instruction: &Instruction, cache: &Arc<Cache>) -> Result<Vec<String>, Box<dyn Error>>;
    // fn will_touch(&self, instruction: &Instruction) -> Result<Vec<String>, Box<dyn Error>> {
    //     let mut result = self.will_write(instruction)?;
    //     let reqire = self.require(instruction);
    //     for (index, data) in reqire.data {
            
    //     }
    //     Ok(result)
    // }
    fn execute(&self, instruction: Instruction, storage_handler: Arc<Cache>, invoke_handler: Arc<Mutex<InvokeHandler>>) -> Result<(), Box<dyn Error>>;
}



pub fn process_builtin_address(address: &str) -> Option<impl ProgramTrait> {
    match address {
        "System" => Some(SystemProgram{}),
        _ => Option::None
    }
}
