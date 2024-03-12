use borsh::BorshDeserialize;

use crate::{
    instruction::Instruction,
    program_result::ProgramResult,
    system_program::system_instruction::SystemInstrusction,
    utils::{
        custom_assert, next_account
    }
};

pub struct SystemProgram {}

impl SystemProgram {
    pub fn process_instruction (
        instruction: Instruction
    ) -> Result<ProgramResult, &'static str> {
        // Extract the command
        let command = SystemInstrusction::try_from_slice(&instruction.data.as_slice());

        match command {
            Ok(command) => {
                match command {
                    SystemInstrusction::Send { amount, receiver_account_create } => {
                        // Define a program result
                        let mut program_result = ProgramResult::default();
                        // Create an account iterator
                        let mut accounts_iter = instruction.accounts.iter();

                        // Extract sender
                        let sender = next_account(&mut accounts_iter)?;
                        // Check sender account props
                        custom_assert(sender.is_signer)?;
                        custom_assert(sender.is_writable)?;
                        custom_assert(sender.underlying_account.owner == instruction.program_id)?;

                        // Extract receiver
                        let receiver = next_account(&mut accounts_iter)?;
                        // Check receiver props
                        custom_assert(receiver.is_writable)?;
                        custom_assert(receiver.underlying_account.owner == instruction.program_id)?;

                        // Check if sender has enough money
                        if sender.underlying_account.atoms < amount {
                            return Err("Not enough money")
                        }

                        // Set the new sender balance
                        program_result.set_atoms(
                            sender.underlying_account.pubkey.clone(),
                            sender.underlying_account.atoms - amount
                        );

                        // Set the new receiver balance
                        program_result.set_atoms(
                            receiver.underlying_account.pubkey.clone(),
                            receiver.underlying_account.atoms + amount
                        );

                        // Return all changes
                        Ok(program_result)
                    },
                    _ => return Err("Unrecognized command error")
                }
            }
            Err(_) => return Err("Parse error")
        }
    }

    pub fn get_address() -> &'static str {
        "System"
    }
}
