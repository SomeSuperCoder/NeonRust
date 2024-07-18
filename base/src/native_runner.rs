use crate::custom_runner::CustomRunner;
use crate::instruction::Instruction;
use crate::invoke_handler::InvokeHandler;
use crate::program_result::ProgramResult;
use crate::system_program::main_struct::SystemProgram;
use std::sync::RwLock;
use config::SYSTEM_PROGRAM_ADDRESS;
use std::sync::Arc;

// The Native Runner is the main program. He is the 👑GOD👑 of the Neon blockchain!
pub struct NativeRunner {}

impl NativeRunner {
    pub fn process_instrcution (
        ins: Instruction,
        _invoke_handler: Arc<RwLock<InvokeHandler>>
    ) -> Result<ProgramResult, &'static str> {
        match ins.program_account.underlying_account.pubkey.as_str() {
            SYSTEM_PROGRAM_ADDRESS => {
                SystemProgram::process_instruction(ins)
            },
            _ => CustomRunner::process_foreign_instruction(ins)
        }
    }
}
