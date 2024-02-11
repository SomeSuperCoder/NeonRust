use std::collections::HashMap;

pub struct Epoch {
    validators_snapshot: HashMap<String, u128>
}

impl Epoch {
    pub fn select(time: u128, prev_blockhash: String) {
        println!("{} {}", time,prev_blockhash)
    }
}
