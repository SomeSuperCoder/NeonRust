mod authority_table;

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use rand_distr::{Distribution, WeightedIndex};
    use std::{hash::{Hash, Hasher}, collections::HashMap};

    use super::authority_table;

    #[test]
    fn rng_test() {
        // Create a list of elements and their weights.
        let elements = vec![
            ("Alice".to_string(), 10.0),
            ("Bob".to_string(), 5.0),
            ("Carol".to_string(), 2.0),
        ];

        // Create a string seed for the random number generator.
        let seed = "my_secret_seedaaa";

        // Create a seeded random number generator.
        let mut table = authority_table::AuthorityTable::new(elements, "aboba".to_string());
        let mut table_iter = table.clone().into_iter();
        for i in 1..100 {
            println!("{:?}", table_iter.next().unwrap());
        }
    }

}
