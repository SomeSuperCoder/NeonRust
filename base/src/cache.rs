use crate::account::{Account, AccountInfo};
use crate::instruction::{InstrcuctionSekelton, Instruction};
use crate::program_result::AccountChange;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Default)]
pub struct Cache {
    pub accounts: HashMap<String, Account>,
    pub read_locks: HashSet<String>,
    pub write_locks: HashSet<String>,
    pub used_signatures: Vec<String>
}

impl Cache {
    pub fn process_change(&mut self, change: AccountChange) -> Result<(), ()> {
        match change {
            AccountChange::SetAtoms { of, amount } => {
                match self.get_owned_account(&of) {
                    Some(mut account) => {
                        account.atoms = amount;

                        self.set_account(account)
                    },
                    None => return Err(())
                }
            },
            AccountChange::SetData { of, data } => {
                match self.get_owned_account(&of) {
                    Some(mut account) => {
                        account.data = data;

                        self.set_account(account)
                    },
                    None => return Err(())
                }
            }
        }

        Ok(())
    }

    pub fn lock(&mut self, accounts: &Vec<AccountInfo>) -> CacheLock {
        let mut cache_lock = CacheLock::default();

        let _: Vec<_> = accounts.into_iter().map(|account| {
            let target;
            let lock;

            if account.is_writable {
                target = &mut self.write_locks;
                lock = &mut cache_lock.wirte_locks;
            } else {
                target = &mut self.read_locks;
                lock = &mut cache_lock.read_locks;
            }

            while target.contains(&account.underlying_account.pubkey) {};

            target.insert(account.underlying_account.pubkey.clone());
            lock.insert(account.underlying_account.pubkey.clone());
        }).collect();

        cache_lock
    }
    
    pub fn release(&mut self, lock: CacheLock) {
        for r_lock in lock.read_locks {
            self.read_locks.remove(&r_lock);
        }

        for w_lock in lock.wirte_locks {
            self.write_locks.remove(&w_lock);
        }
    }

    pub fn form_instruction(&self, instruction_skeleton: InstrcuctionSekelton) -> Result<Instruction, ()> {
        let mut result_accounts = Vec::new();
        for account_skeleton in instruction_skeleton.accounts {
            match self.get_owned_account(&account_skeleton.pubkey) {
                Some(account) => {
                    result_accounts.push(
                        AccountInfo {
                            underlying_account: account,
                            is_signer: account_skeleton.is_signer,
                            is_writable: account_skeleton.is_writable
                        }
                    )
                },
                None => return Err(())
            }
        }

        Ok(
            Instruction {
                program_id: instruction_skeleton.program_id,
                data: instruction_skeleton.data,
                accounts: result_accounts
            }
        )
    }

    pub fn get_owned_account(&self, pubkey: &String) -> Option<Account> {
        if let Ok(account_data) = fs::read_to_string(make_account_path(pubkey)) {
            Some(serde_json::from_str(account_data.as_str()).unwrap())
        } else {
            None
        }
    }

    pub fn set_account(&self, account: Account) {
        fs::write(make_account_path(&account.pubkey), serde_json::to_string(&account).unwrap()).unwrap();
    }

    pub fn does_this_account_exist(&self, pubkey: &String) -> bool {
        fs::read_to_string(make_account_path(pubkey)).is_ok()
    }
} 

#[derive(Default)]
pub struct CacheLock {
    pub read_locks: HashSet<String>,
    pub wirte_locks: HashSet<String>
}

fn make_account_path(pubkey: &String) -> String {
    format!("./neon_validator/cache/accounts/{}", pubkey)
}
