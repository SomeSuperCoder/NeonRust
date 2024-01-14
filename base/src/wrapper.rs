use crate::account::Account;

struct Wrapper<T> {
    program: T,
    accounts: Account
}
