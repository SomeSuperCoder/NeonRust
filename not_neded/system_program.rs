use crate::instruction::Instruction;
use crate::mutable_storage::MutableStorage;
use crate::program::ProgramTrait;
use crate::second_runtime::{InvokeHandler, ProgramRequire};
use crate::transaction::Transaction;
use crate::ecdsa;
use std::error::Error;
use std::sync::{Arc, Mutex};
use crate::cache::Cache;

pub struct SystemProgram {}

impl SystemProgram {
    // FORMAT: send~amount:int;to:String;create_account:bool
    pub fn parse_instruction(tx: Instruction) -> Result<(String, Vec<String>), Box<dyn Error>> {
        let mut parts = tx.message_text.split('~');
        let command = parts.next().ok_or("Missing command")?;
        let args = parts.next().ok_or("Missing arguments")?;
        let args: Vec<String> = args.split(';').map(|arg| arg.to_string()).collect();

        if args.len() != 3 {
            return Err("Invalid number of arguments")?;
        }

        Ok((command.to_string(), args))
    }
}

impl ProgramTrait for SystemProgram {
    fn will_write(&self, instruction: &Instruction, cache: &Arc<Cache>) -> Result<Vec<String>, Box<dyn Error>> {
        let (command, args) = SystemProgram::parse_instruction(instruction.clone())?;
        let mut result: Vec<String> = Vec::new();

        if command == "send" {
            result.push(
                format!("/System/{}/balance", args[1])
            );
            result.push(
                format!("/System/{}/balance",
                    &instruction.sender
                )
            );
        } else {
            Err("Unknown command")?
        }

        Ok(result)
    }

    fn will_read(&self, instruction: &Instruction, cache: &Arc<Cache>) -> Result<ProgramRequire, Box<dyn Error>> {
        let (command, args) = SystemProgram::parse_instruction(instruction.clone())?;
        // let mut result: Vec<String> = Vec::new();
        let result;
        
        if command == "send" {
            result = ProgramRequire::new(
                vec![
                    (format!("/System/{}/balance", args[1]), "0".to_string()),
                    (format!("/System/{}/balance", &instruction.sender), "0".to_string())
                ]
            );
        } else {
            result = ProgramRequire::new(Vec::new());
            Err("Unknown command")?
        }

        Ok(result)
    }

    fn execute(&self, instruction: Instruction, storage_handler: Arc<Cache>, invoke_handler: Arc<Mutex<InvokeHandler>>) -> Result<(), Box<dyn Error>> {
        // let (command, args) = SystemProgram::parse_tx(tx.clone()).unwrap();

        // if command == "send" {
        //     // result.push(
        //     //     format!("/System/{}/balance", args[1])
        //     // );
        //     // result.push(
        //     //     format!("/System/{}/balance",
        //     //             ecdsa::public_key_to_address(
        //     //                 &tx.sender_part.public_key
        //     //             )
        //     //     )
        //     // );
            
        //     // check if sender.balance >= amount
        //     // sender.balance -= amount
        //     // receiver.balance += amount
            
        //     let sender_balance = storage.get("".to_string()).or(Some(&"0".to_string())).unwrap();
        //     let sender_balance: usize = sender_balance.parse::<usize>()?;
            
        // };
        // Ok(());
        todo!();
    }
}
