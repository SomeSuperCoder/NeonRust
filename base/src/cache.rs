use crate::account::{Account, AccountInfo};
use crate::program_result::AccountChange;
use std::collections::{HashMap, HashSet};
use std::default;
use std::sync::Mutex;

#[derive(Default)]
pub struct Cache {
    pub accounts: Mutex<HashMap<String, Account>>,
    pub read_locks: Mutex<HashSet<String>>,
    pub write_locks: Mutex<HashSet<String>>
}

impl Cache {
    pub fn process_change(&mut self, change: AccountChange) -> Result<(), ()> {
        match change {
            AccountChange::SetAtoms { of, amount } => {
                match self.accounts.lock().unwrap().get_mut(&of) {
                    Some(account) => {
                        account.atoms = amount;
                    },
                    None => {}
                }
            },
            AccountChange::SetData { of, data } => {
                match self.accounts.lock().unwrap().get_mut(&of) {
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
            let mut target;
            let lock;

            if account.is_writable {
                target = self.write_locks.lock().unwrap();
                lock = &mut cache_lock.wirte_locks;
            } else {
                target = self.read_locks.lock().unwrap();
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
            self.read_locks.lock().unwrap().remove(&r_lock);
        }

        for w_lock in lock.wirte_locks {
            self.write_locks.lock().unwrap().remove(&w_lock);
        }
    }
} 

#[derive(Default)]
pub struct CacheLock {
    pub read_locks: HashSet<String>,
    pub wirte_locks: HashSet<String>
}
