use crate::blockchain::Blockchain;
use crate::cache::Cache;
use crate::block::Block;

pub struct BlockchainWrapper {
    blockchain: Blockchain,
    cache: Cache
}

impl BlockchainWrapper {
    fn add_block_and_cache(block: Block) {
        // here we need to loop over all txes in the block PoH seq and cahe their result_part.storage
    }
}
