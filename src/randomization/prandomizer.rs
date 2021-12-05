use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

// Magi Pseudo-random number generator

// A class for which each object accepts
// a number in its constructor and uses it to
// create the internal generator object

pub struct PRandomizer {
    generator: Pcg64,
    lower_limit: u64,
    upper_limit: u64,
}

impl PRandomizer {
    pub fn new(seed: u64) -> Self {
        Self {
            generator: Pcg64::seed_from_u64(seed),
            lower_limit: 0_u64,
            upper_limit: u64::MAX,
        }
    }

    #[allow(dead_code)]
    pub fn new_limited(seed: u64, from: u64, to: u64) -> Self {
        Self {
            generator: Pcg64::seed_from_u64(seed),
            lower_limit: from,
            upper_limit: to,
        }
    }

    pub fn get(&mut self) -> u64 {
        self.generator.gen_range(self.lower_limit..self.upper_limit)
    }
}

impl Iterator for PRandomizer {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get())
    }
}

#[cfg(test)]
mod tests {
    use super::PRandomizer;

    #[test]
    fn prandom_generator_is_deterministic() {
        let mut gen1: PRandomizer = PRandomizer::new(123);
        let mut gen2: PRandomizer = PRandomizer::new(123);
        assert_eq!(gen1.get(), gen2.get());
    }

    #[test]
    fn prandom_generator_is_seedable() {
        let mut gen1: PRandomizer = PRandomizer::new(123);
        let mut gen2: PRandomizer = PRandomizer::new(124);
        assert_ne!(gen1.get(), gen2.get());
    }

    #[test]
    fn zero_is_valid_seed_for_prandom_generator() {
        let mut gen: PRandomizer = PRandomizer::new(0);
        assert_ne!(gen.get(), 0);
    }

    #[test]
    fn prandom_generator_generates_numbers_within_limits() {
        const MIN_LIMIT: u64 = 10_u64;
        const MAX_LIMIT: u64 = 20_u64;
        let mut gen: PRandomizer = PRandomizer::new_limited(123, MIN_LIMIT, MAX_LIMIT);
        for _ in 0..11 {
            let generated = gen.get();
            assert!(generated >= MIN_LIMIT);
            assert!(generated < MAX_LIMIT);
        }
    }
}
