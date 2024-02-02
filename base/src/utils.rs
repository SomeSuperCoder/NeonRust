use crate::account::AccountInfo;

pub fn custom_assert(traget: bool) -> Result<(), &'static str> {
    if traget {
        Ok(())
    } else {
        Err("Custom assert error")
    }
}

pub fn next_account<'a>(mut iter: impl Iterator<Item = &'a AccountInfo>) -> Result<&'a AccountInfo, &'static str> {
    match iter.next() {
        Some(value) => Ok(value),
        None => Err("Error getting account")
    }
}
