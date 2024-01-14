use crate::transaction::Transaction;
use crate::hasher;
#[derive(Debug, Default)]
pub struct History {
    pub seq: Vec<Part>
}

impl History{
    pub fn add_part(&mut self, part: Part) {
        self.seq.push(
            part
        );
    }

    pub fn create_new_part(&self, tx: Option<Transaction>) -> Part {
        let hash: String = hasher::hash_string(format!("{:?}", tx));
        let prev_hash: String = self.get_latest_hash();
        Part{
            hash,
            prev_hash,
            tx
        }
    }

    pub fn get_latest_hash(&self) -> String {
        let pev_elem = self.seq.last();
        match pev_elem {
            Some(data) => data.hash.clone(),
            None => String::from("0000000000000000000000000000000000000000000000000000000000000000")
        }
    }

    pub fn new() -> History {
        History {
            seq: Vec::new()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Part {
    pub hash: String,
    pub prev_hash: String,
    pub tx: Option<Transaction>
}
