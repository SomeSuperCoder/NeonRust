use crate::account::{Account, AccountInfo};
use crate::instruction::{InstrcuctionSekelton, Instruction};
use crate::program_result::AccountChange;
use std::collections::HashSet;
use std::fs;

#[derive(Default)]
pub struct Cache {
    pub read_locks: HashSet<String>,
    pub write_locks: HashSet<String>
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
            },
            AccountChange::CloseAccount { pubkey } => {
                if let Ok(_) = fs::remove_file(make_account_path(&pubkey)) {} else {
                    return Err(())
                }
            },
            AccountChange::CreateAccount { account } => {
                if !self.does_this_account_exist(&account.pubkey) {
                    self.set_account(account)
                } else {
                    return Err(())
                }
            },
            AccountChange::SetExecutable { of, executable } => {
                match self.get_owned_account(&of) {
                    Some(mut account) => {
                        account.executable = executable;

                        self.set_account(account)
                    },
                    None => return Err(())
                }
            },
            AccountChange::StartValidating { account } => {
                self.add_validator(account);
            },
            AccountChange::StopValidating { account } => {
                self.remove_validator(account);
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
        let program_account;

        if let Some(program_account_inner) = self.get_owned_account(&instruction_skeleton.program_id) {
            program_account = AccountInfo {
                is_signer: false,
                is_writable: false,
                underlying_account: program_account_inner
            };
        } else {
            return Err(())
        }

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
                data: instruction_skeleton.data,
                accounts: result_accounts,
                program_account: program_account
            }
        )
    }

    pub fn get_owned_account(&self, pubkey: &String) -> Option<Account> {
        if *pubkey == String::from(config::SYSTEM_PROGRAM_ADDRESS) {
            return Some(
                Account {
                    admin: true,
                    atoms: 0,
                    authority: 0,
                    data: Vec::new(),
                    executable: true,
                    owner: String::from(config::SYSTEM_PROGRAM_ADDRESS),
                    pubkey: String::from(config::SYSTEM_PROGRAM_ADDRESS)
                }
            )
        }
        
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

    pub fn spend(&self, signature: Vec<u8>) {
        let hash = crate::hasher::hash_bytes(&signature.as_slice());
        fs::write(make_spend_path(&hash), &[]).unwrap();
    }

    pub fn is_spent(&self, signature: Vec<u8>) -> bool {
        let hash = crate::hasher::hash_bytes(&signature.as_slice());
        fs::read_to_string(make_spend_path(&hash)).is_ok()
    }

    pub fn get_validator_list(&self) -> Vec<Account> {
        let mut result = Vec::new();

        for entry in fs::read_dir("./neon_validator/cache/validators/").unwrap() {
            let entry = entry.unwrap();

            let pubkey = entry.path().file_name().unwrap().to_str().unwrap().to_string();
            let account = self.get_owned_account(&pubkey);

            if let Some(account) = account {
                result.push(account);
            }
        }

        result
    }

    pub fn add_validator(&self, who: Account) {
        fs::write(make_validator_path(&who.pubkey), "").unwrap();
    }

    pub fn remove_validator(&self, who: Account) {
        let _ = fs::remove_file(make_validator_path(&who.pubkey));
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

fn make_spend_path(hash: &String) -> String {
    format!("./neon_validator/cache/signatures/{}", hash)
}

fn make_validator_path(pubkey: &String) -> String {
    format!("./neon_validator/cache/validators/{}", pubkey)
}

// if let Ok(entries) = fs::read_dir(dir_path) {
//     for entry in entries {
//         if let Ok(entry) = entry {
//             if entry.file_type().unwrap().is_file() {
//                 file_count += 1;
//             }
//         }
//     }
// }
