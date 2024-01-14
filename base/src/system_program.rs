use crate::program::ProgramTrait;
use crate::transaction::Transaction;
use crate::ecdsa;
use std::error::Error;

pub struct SystemProgram {}

impl SystemProgram {
    // FORMAT: send~amount:int;to:String;create_account:bool
    pub fn parse_tx(tx: Transaction) -> Result<(String, Vec<String>), Box<dyn Error>> {
        let mut parts = tx.sender_part.message_text.split('~');
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
    fn will_touch(&self, tx: Transaction) -> Result<Vec<String>, Box<dyn Error>> {
        let parse_result = SystemProgram::parse_tx(tx.clone())?;
        let args = &parse_result.1;
        let mut result: Vec<String> = Vec::new();

        if parse_result.0 == "send" {
            result.push(
                format!("/System/{}/balance", args[1])
            );
            result.push(
                format!("/System/{}/balance",
                        ecdsa::public_key_to_address(
                            &tx.sender_part.public_key
                        )
                )
            );
        } else {
            Err("Unknown command")?
        }

        Ok(result)
    }

    fn execute(&self, tx: Transaction) {
        todo!()
    }
}
