use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, WeightedIndex};
use std::hash::{Hash, Hasher};

// Define a function to hash any text.
fn hash_text(text: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

#[derive(Default, Clone)]
pub struct AuthorityTable {
    pub data: Vec<(String, f32)>,
    seed: String,
    index: usize
}

impl AuthorityTable {
    pub fn new(data: Vec<(String, f32)>, initial_seed: String) -> Self {
        AuthorityTable {
            data: data,
            seed: initial_seed,
            index: 0
        }
    }
}

impl Iterator for AuthorityTable {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let cloned = self.data.clone();
        let converted = cloned.iter().collect::<Vec<_>>();
        // Create a seeded random number generator.
        let mut rng = rand::rngs::StdRng::seed_from_u64(hash_text(self.seed.as_str()));

        // Create a weighted index distribution from the elements and weights.
        let dist = WeightedIndex::new(converted.iter().map(|(_, weight)| *weight)).unwrap();

        // Choose a random element from the distribution.
        let chosen_element = &converted[dist.sample(&mut rng)].0;
        self.seed = chosen_element.to_owned() + self.index.to_string().as_str();
        self.index += 1;
        Some(chosen_element.to_owned())
    }
}
