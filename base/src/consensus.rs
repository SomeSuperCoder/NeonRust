use crate::{blockchain::Blockchain, cache::Cache};
use rand_distr::{Distribution, WeightedIndex};
use rand_core::SeedableRng;
use std::hash::{Hash, Hasher};
use config::GENESIS_PUBKEY;

pub struct Consensus {}

impl Consensus {
    fn hash_text(text: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        text.hash(&mut hasher);
        hasher.finish()
    }

    
    pub fn select(blockchain: &Blockchain, cache: &Cache) -> String {
        let entropy = blockchain.get_latest_hash();
        let entropy = Self::hash_text(entropy.as_str());

        let validators = cache.get_validator_list();

        let mut rng = rand::rngs::StdRng::seed_from_u64(entropy);

        if let Ok(dist) = WeightedIndex::new(validators.iter().map(|acc| acc.atoms)) {
            validators[dist.sample(&mut rng)].pubkey.clone()
        } else {
            println!("Using GENESIS Validator!");
            String::from(GENESIS_PUBKEY)
        }
    }
}
