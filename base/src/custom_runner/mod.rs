mod sc_rpc;

use crate::instruction::Instruction;
use borsh::BorshDeserialize;
use sc_rpc::SCRPC;
use crate::program_result::ProgramResult;
use tokio::time::timeout;
use tokio;
use mlua::{Lua, LuaOptions, StdLib};
use mlua::prelude::LuaError;

pub struct CustomRunner {}

impl CustomRunner {
    pub fn process_foreign_instruction (
        instruction: Instruction
    ) -> Result<ProgramResult, &'static str> {
        if let Ok(args) = SCRPC::try_from_slice(&instruction.data) {
            let program_result = ProgramResult::default();

            if let Ok(user_code) = String::from_utf8(instruction.program_account.data) {
                let result = tokio::runtime::Runtime::new().unwrap().block_on(
                    timeout(std::time::Duration::from_secs(3), execute_user_code(user_code))
                );

                if let Ok(result2) = result {
                    match result2 {
                        Err(lua_error) => {
                            let lua_error_string = lua_error.to_string();
                            println!("User code failed with error: {}", lua_error_string);
                            
                            return Err("User code failed")
                        },
                        _ => {

                        }
                    }
                } else {
                    println!("User code ran out of time");

                    return Err("User code ran out of time")
                }
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

async fn execute_user_code(code: String) -> Result<(), LuaError> {
    let lua = Lua::new_with(StdLib::NONE, LuaOptions::new()).unwrap();

    lua.load(code).exec()
}
