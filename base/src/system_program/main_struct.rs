use borsh::BorshDeserialize;

use crate::{
    account::Account, ecdsa::{address_to_public_key, public_key_to_address}, instruction::Instruction, program_result::{AccountChange, ProgramResult}, system_program::system_instruction::SystemInstrusction, utils::{
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
                    SystemInstrusction::Send { amount } => {
                        for i in 0..100 {
                            println!("We are in seeend!");
                        }
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
                        program_result.changes.push(
                            AccountChange::SetAtoms {
                                of: sender.underlying_account.pubkey.clone(),
                                amount: sender.underlying_account.atoms - amount
                            }
                        );

                        // Set the new receiver balance
                        program_result.changes.push(
                            AccountChange::SetAtoms {
                                of: receiver.underlying_account.pubkey.clone(),
                                amount: receiver.underlying_account.atoms + amount
                            }
                        );

                        // Return all changes
                        Ok(program_result)
                    },
                    SystemInstrusction::HelloWorld => {
                        for i in 0..10 {
                            println!("Hello, World!")
                        };
                        Ok(ProgramResult::default())
                    },
                    SystemInstrusction::CloseAccount => {
                        let mut program_result = ProgramResult::default();
                        let mut accounts_iter = instruction.accounts.iter();

                        let owner = next_account(&mut accounts_iter)?;
                        custom_assert(owner.is_signer)?;
                        let target = next_account(&mut accounts_iter)?;
                        custom_assert(target.is_writable)?;
                        custom_assert(target.underlying_account.owner == owner.underlying_account.pubkey)?;

                        program_result.changes.push(
                            AccountChange::CloseAccount { pubkey: target.underlying_account.pubkey.clone() }
                        );

                        Ok(program_result)
                    },
                    SystemInstrusction::CreateAccount { pubkey } => {
                        let mut program_result = ProgramResult::default();
                        let mut accounts_iter = instruction.accounts.iter();

                        if let Ok(_) = address_to_public_key(pubkey.clone()) {} else {
                            return Err("Invalid public key")
                        }

                        let owner = next_account(&mut accounts_iter)?;
                        custom_assert(owner.is_signer)?;

                        let mut new_account = Account::default();
                        new_account.owner = owner.underlying_account.pubkey.clone();
                        new_account.pubkey = pubkey;

                        program_result.changes.push(
                            AccountChange::CreateAccount { account: new_account }
                        );

                        Ok(program_result)
                    },
                    SystemInstrusction::SetAdmin { admin } => {
                        let mut program_result = ProgramResult::default();
                        let mut accounts_iter = instruction.accounts.iter();

                        let dude = next_account(&mut accounts_iter)?;
                        custom_assert(dude.is_signer)?;
                        custom_assert(dude.underlying_account.admin)?;

                        let target = next_account(&mut accounts_iter)?;
                        custom_assert(target.is_writable)?;
                        custom_assert(target.underlying_account.owner == instruction.program_id)?;

                        program_result.changes.push(
                            AccountChange::SetAdmin { of: dude.underlying_account.pubkey.clone(), admin: admin }
                        );

                        Ok(program_result)
                    },
                    SystemInstrusction::SetAuthority { authority } => {
                        let mut program_result = ProgramResult::default();
                        let mut accounts_iter = instruction.accounts.iter();

                        let dude = next_account(&mut accounts_iter)?;
                        custom_assert(dude.is_signer)?;
                        custom_assert(dude.underlying_account.admin)?;

                        let target = next_account(&mut accounts_iter)?;
                        custom_assert(target.is_writable)?;
                        custom_assert(target.underlying_account.owner == instruction.program_id)?;

                        program_result.changes.push(
                            AccountChange::SetAuthority { of: target.underlying_account.pubkey.clone(), authority: authority }
                        );

                        Ok(program_result)
                    }
                }
            }
            Err(_) => return Err("Parse error")
        }
    }

    pub fn get_address() -> &'static str {
        "System"
    }
}
