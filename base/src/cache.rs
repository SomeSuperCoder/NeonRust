use crate::account::{Account, AccountInfo};
use crate::instruction::{InstrcuctionSekelton, Instruction};
use crate::program_result::AccountChange;
use std::collections::{HashMap, HashSet};
use std::default;
use std::sync::Mutex;

#[derive(Default)]
pub struct Cache {
    pub accounts: HashMap<String, Account>,
    pub read_locks: HashSet<String>,
    pub write_locks: HashSet<String>
}

impl Cache {
    pub fn process_change(&mut self, change: AccountChange) -> Result<(), ()> {
        match change {
            AccountChange::SetAtoms { of, amount } => {
                match self.accounts.get_mut(&of) {
                    Some(account) => {
                        account.atoms = amount;
                    },
                    None => {}
                }
            },
            AccountChange::SetData { of, data } => {
                match self.accounts.get_mut(&of) {
                    Some(account) => {
                        account.data = data;
                    },
                    None => {}
                }
            }
        }

        Ok(())
    }

    // fn get_account(&mut self, key: &String) -> Result<MutexGuard<'_, Account>, ()> {
    //     let mut access = self.accounts.lock().unwrap();
    //     match access.get_mut(key) {
    //         Some(account) => Ok(account),
    //         None => Err(())
    //     }
    // }

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
            match self.get_owned_account(account_skeleton.pubkey) {
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

    pub fn get_owned_account(&self, pubkey: String) -> Option<Account> {
        match self.accounts.get(&pubkey) {
            Some(account_ref) => Some(account_ref.clone()),
            None => None
        }
    }
} 

#[derive(Default)]
pub struct CacheLock {
    pub read_locks: HashSet<String>,
    pub wirte_locks: HashSet<String>
}
