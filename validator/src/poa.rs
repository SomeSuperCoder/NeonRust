use std::collections::HashMap;
use std::fs;

pub struct PoA {}

impl PoA {
    pub fn next() -> String {
        use rand::distributions::{Distribution, WeightedIndex};
        use rand::thread_rng;

        // Create a hashmap with names and their weights
        let names_and_weights: HashMap<String, u128> = Self::make_authority_hashmap();

        // Create a weighted index distribution from the hashmap
        if let Ok(dist) = WeightedIndex::new(names_and_weights.values().cloned()) {
            // Generate a random number using the distribution
            let mut rng = thread_rng();
            let index = dist.sample(&mut rng);

            // Get the name corresponding to the random index
            let name = names_and_weights.keys().nth(index).unwrap();

            // Print the selected name
            name.clone()
        } else {
            return String::from(config::GENESIS_PUBKEY)
        }
        
    }

    pub fn make_authority_hashmap() -> HashMap<String, u128> {
        let mut hm = HashMap::new();

        for entry in fs::read_dir("./neon_validator/cache/authority").unwrap() {
            let entry = entry.unwrap();
            let pubkey = entry.path().file_name().unwrap().to_str().unwrap().to_string();
            let authority: u128 = fs::read_to_string(entry.path()).unwrap().parse().unwrap();
            hm.insert(pubkey, authority);
        }

        hm
    }
}