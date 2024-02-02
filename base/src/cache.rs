use crate::account::{Account, AccountInfo};
use crate::program_result::AccountChange;
use std::collections::{HashMap, HashSet};
use std::default;

enum MutOrNot<'a, T> {
    Mutable(Option<&'a mut T>),
    Immutable(Option<&'a T>),
}

pub struct Cache {
    accounts: HashMap<String, Account>,
    read_locks: HashSet<String>,
    write_locks: HashSet<String>
}

impl Cache {
    pub fn process_change(&mut self, change: AccountChange) -> Result<(), ()> {
        match change {
            AccountChange::SetAtoms { of, amount } => {
                let account = self.get_account(&of)?;
                account.atoms = amount;
            },
            AccountChange::SetData { of, data } => {
                let account = self.get_account(&of)?;
                account.data = data;
            }
        }

        Ok(())
    }

    fn get_account(&mut self, key: &String) -> Result<&mut Account, ()> {
        match self.accounts.get_mut(key) {
            Some(account) => Ok(account),
            None => Err(())
        }
    }

    pub fn lock(&mut self, accounts: Vec<AccountInfo>) -> CacheLock {
        let mut cache_lock = CacheLock::default();

        accounts.into_iter().map(|account| {
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

            vec![target, lock].into_iter().map(|lock_part| {
                lock_part.insert(account.underlying_account.pubkey);
            });
        });

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
} 

#[derive(Default)]
pub struct CacheLock {
    pub read_locks: HashSet<String>,
    pub wirte_locks: HashSet<String>
}
