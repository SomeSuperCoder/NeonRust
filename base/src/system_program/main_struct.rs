use crate::{
    account::Account, ecdsa::TriplePublicKey, instruction::Instruction, program_result::{AccountChange, ProgramResult}, system_program::system_instruction::SystemInstrusction, utils::{
        custom_assert, next_account
    }
};

pub struct SystemProgram {}

impl SystemProgram {
    pub fn process_instruction (
        instruction: Instruction
    ) -> Result<ProgramResult, &'static str> {

        for _ in 0..100 {
            println!("We are in system!");
        }

        // Extract the command
        let command = serde_json::from_str(&instruction.data);
        
        for _ in 0..100 {
            println!("We are in system 2222222222222222!");
        }

        match command {
            Ok(command) => {
                match command {
                    SystemInstrusction::Send { amount } => {
                        // Define a program result
                        let mut program_result = ProgramResult::default();
                        // Create an account iterator
                        let mut accounts_iter = instruction.accounts.iter();
                        println!("Here1!");
                        // Extract sender
                        let sender = next_account(&mut accounts_iter)?;
                        println!("Here2!");
                        // Check sender account props
                        custom_assert(sender.is_signer)?;
                        println!("Cond1");
                        custom_assert(sender.underlying_account.owner == instruction.program_account.underlying_account.pubkey)?;
                        println!("Cond2");
                        
                        // Extract receiver
                        let receiver = next_account(&mut accounts_iter)?;
                        // Check receiver props
                        custom_assert(receiver.is_writable)?;
                        println!("Cond3");
                        custom_assert(receiver.underlying_account.owner == instruction.program_account.underlying_account.pubkey)?;
                        println!("Cond4");

                        // Check if sender has enough money
                        if sender.underlying_account.atoms < amount {
                            return Err("Not enough money")
                        }
                        println!("Cond5");

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
                        for _ in 0..10 {
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

                        if let None = TriplePublicKey::from_address(pubkey.clone()) {
                            return Err("Invalid public key")
                        }

                        let owner = next_account(&mut accounts_iter)?;
                        custom_assert(owner.underlying_account.executable)?;
                        custom_assert(owner.is_signer)?;

                        let mut new_account = Account::default();
                        new_account.owner = owner.underlying_account.pubkey.clone();
                        new_account.pubkey = pubkey;

                        program_result.changes.push(
                            AccountChange::CreateAccount { account: new_account }
                        );

                        Ok(program_result)
                    },
                    SystemInstrusction::CreateSystemAccount { pubkey } => {
                        let mut program_result = ProgramResult::default();

                        if let None = TriplePublicKey::from_address(pubkey.clone()) {
                            return Err("Invalid public key")
                        }

                        let mut new_account = Account::default();
                        new_account.owner = String::from(config::SYSTEM_PROGRAM_ADDRESS);
                        new_account.pubkey = pubkey;

                        program_result.changes.push(
                            AccountChange::CreateAccount { account: new_account }
                        );

                        Ok(program_result)
                    },
                    SystemInstrusction::Validate => {
                        let mut program_result = ProgramResult::default();

                        Ok(program_result)
                    },
                    SystemInstrusction::Unvalidate => {
                        let mut program_result = ProgramResult::default();

                        Ok(program_result)
                    }
                }
            }
            Err(_) => return Err("Parse error")
        }
    }
}
