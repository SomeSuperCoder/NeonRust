use crate::custom_runner::CustomRunner;
use crate::instruction::Instruction;
use crate::invoke_handler::InvokeHandler;
use crate::program_result::ProgramResult;
use crate::system_program::main_struct::SystemProgram;
use std::{error::Error, sync::RwLock};
use std::fmt;
use config::SYSTEM_PROGRAM_ADDRESS;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}


// The Native Runner is the main program. He is the 👑GOD👑 of the Neon blockchain!
pub struct NativeRunner {}

impl NativeRunner {
    pub fn process_instrcution (
        ins: Instruction,
        invoke_handler: Arc<RwLock<InvokeHandler>>
    ) -> Result<ProgramResult, &'static str> {

        match ins.program_account.underlying_account.pubkey.as_str() {
            SYSTEM_PROGRAM_ADDRESS => {
                SystemProgram::process_instruction(ins)
            },
            _ => CustomRunner::process_foreign_instruction(ins)
        }
    }
}
