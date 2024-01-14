use crate::history::History;

#[derive(Debug, Default)]
pub struct Block {
    pub seq: History,
    pub latest_seq_hash: String
}
