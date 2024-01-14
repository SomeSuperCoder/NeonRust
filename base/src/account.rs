use crate::has_address::HasAddress;
use crate::mutable_storage::MutableStorage;
use crate::ecdsa::public_key_to_address;


pub struct Account {
    public_key: Vec<u8>,
    storage: MutableStorage
}

impl HasAddress for Account {
    fn get_address(&self) -> String {
        // encode(&self.public_key).into_string()
        public_key_to_address(&self.public_key)
    }
}
