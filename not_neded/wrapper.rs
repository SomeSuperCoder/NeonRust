use crate::blockchain::Blockchain;
use crate::cache::Cache;
use crate::second_runtime::Runtime;
use std::sync::Arc;

pub struct Wrapper {
    pub blockchain: Arc<Blockchain>,
    pub cache: Arc<Cache>,
    pub runtime: Arc<Runtime>
}
