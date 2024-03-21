mod sc_rpc;

use crate::instruction::Instruction;
use borsh::BorshDeserialize;
use sc_rpc::SCRPC;
use crate::program_result::ProgramResult;

pub struct CustomRunner {}

impl CustomRunner {
    pub fn process_foreign_instruction (
        instruction: Instruction
    ) -> Result<ProgramResult, &'static str> {
        if let Ok(args) = SCRPC::try_from_slice(&instruction.data) {
            let program_result = ProgramResult::default();

            if let Ok(user_code) = String::from_utf8(instruction.program_account.data) {
                println!("Executing user code!")
            } else {
                println!("Failed to load user code. UTF8 decode error");

                return Err("Failed to load user code. UTF8 decode error")
            }

            Ok(program_result)
        } else {
            Err("Unable to load data")
        }
    }
}
