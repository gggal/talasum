use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

/// Thin wrapper around a pseudo-random generator
pub struct PRandomizer {
    generator: Pcg64,
}

/// To be implemented for random number generators
pub trait Randomizer {
    /// Returns the next randomly generated number
    fn get(&mut self) -> u64;
}

impl PRandomizer {
    /// Returns a PRG instance based on a `seed`
    pub fn new(seed: u64) -> Self {
        Self {
            generator: Pcg64::seed_from_u64(seed),
        }
    }
}

impl Randomizer for PRandomizer {
    /// Returns the next pseudo-randomly generated number
    fn get(&mut self) -> u64 {
        self.generator.gen_range(u64::MIN..u64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::{PRandomizer, Randomizer};

    #[test]
    fn pseudo_random_generator_is_deterministic() {
        let mut gen1: PRandomizer = PRandomizer::new(123);
        let mut gen2: PRandomizer = PRandomizer::new(123);
        assert_eq!(gen1.get(), gen2.get());
    }

    #[test]
    fn pseudo_random_generator_is_seedable() {
        let mut gen1: PRandomizer = PRandomizer::new(123);
        let mut gen2: PRandomizer = PRandomizer::new(124);
        assert_ne!(gen1.get(), gen2.get());
    }

    #[test]
    fn zero_is_valid_seed_for_pseudo_random_generator() {
        let mut gen: PRandomizer = PRandomizer::new(0);
        assert_ne!(gen.get(), 0);
    }

    #[test]
    fn generator_is_iterable() {
        let mut gen: PRandomizer = PRandomizer::new(0);
        assert_ne!(gen.get(), gen.get());
    }
}
