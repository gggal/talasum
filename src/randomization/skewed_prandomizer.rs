use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

pub struct SkewedPRandomizer {
    generator: Pcg32,
    lower_limit: u32,
    upper_limit: u32,
    peeks: Vec<u32>,
}

impl SkewedPRandomizer {
    pub fn new(seed: u64, mut peeks: Vec<u32>) -> Self {
        peeks.sort_unstable();
        Self {
            generator: Pcg32::seed_from_u64(seed),
            lower_limit: 0_u32,
            upper_limit: u32::MAX,
            peeks,
        }
    }

    pub fn new_limited(seed: u64, from: u32, to: u32, mut peeks: Vec<u32>) -> Self {
        peeks.sort_unstable();
        Self {
            generator: Pcg32::seed_from_u64(seed),
            lower_limit: from,
            upper_limit: to,
            peeks,
        }
    }

    fn abs_subtract(left: u32, right: u32) -> u32 {
        if left > right {
            left - right
        } else {
            right - left
        }
    }

    fn find_closest_peek(&self, num: u32) -> Option<u32> {
        if self.peeks.is_empty() {
            None
        } else if self.peeks.len() == 1 {
            Some(self.peeks[0])
        } else {
            let idx = self.peeks.len() as u32 - 1;
            for idx in 0..self.peeks.len() - 1 {
                if Self::abs_subtract(self.peeks[idx], num)
                    < Self::abs_subtract(self.peeks[idx + 1], num)
                {
                    break;
                }
            }
            Some(idx)
        }
    }

    pub fn get(&mut self) -> u32 {
        let next: u32 = self.generator.gen_range(self.lower_limit..self.upper_limit);
        let controller: u32 = self.generator.gen_range(self.lower_limit..self.upper_limit);

        match (
            self.find_closest_peek(next),
            self.find_closest_peek(controller),
        ) {
            (Some(peek1), Some(peek2)) => {
                let dist1 = Self::abs_subtract(next, peek1);
                let dist2 = Self::abs_subtract(controller, peek2);
                if dist2 < dist1 {
                    controller
                } else {
                    next
                }
            }
            _ => next,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SkewedPRandomizer;

    #[test]
    fn skewed_prandom_generator_is_deterministic() {
        let mut gen1 = SkewedPRandomizer::new(123, Vec::new());
        let mut gen2 = SkewedPRandomizer::new(123, Vec::new());
        assert_eq!(gen1.get(), gen2.get());
    }

    #[test]
    fn skewed_prandom_generator_is_seedable() {
        let mut gen1 = SkewedPRandomizer::new(123, Vec::new());
        let mut gen2 = SkewedPRandomizer::new(124, Vec::new());
        assert_ne!(gen1.get(), gen2.get());
    }

    #[test]
    fn zero_is_valid_seed_for_skewed_prandom_generator() {
        let mut gen = SkewedPRandomizer::new(0, Vec::new());
        assert_ne!(gen.get(), 0);
    }

    #[test]
    fn skewed_prandom_generator_generates_numbers_within_limits() {
        const MIN_LIMIT: u32 = 10_u32;
        const MAX_LIMIT: u32 = 20_u32;
        let mut gen = SkewedPRandomizer::new_limited(123, MIN_LIMIT, MAX_LIMIT, Vec::new());
        for _ in 0..11 {
            let generated = gen.get();
            assert!(generated >= MIN_LIMIT);
            assert!(generated < MAX_LIMIT);
        }
    }

    #[test]
    fn skewed_prandom_generator_is_skewed() {
        let mut gen1 = SkewedPRandomizer::new_limited(123, 0, 1000, vec![100]);
        let mut gen2 = SkewedPRandomizer::new_limited(123, 0, 1000, Vec::new());
        let mut is_skewed = false;
        for _ in 0..100 {
            if gen1.get() != gen2.get() {
                is_skewed = true;
                break;
            }
        }
        assert!(is_skewed);
    }

    #[test]
    fn show_output() {
        let mut gen1 = SkewedPRandomizer::new_limited(123, 0, 1000, vec![500]);
        let mut gen2 = SkewedPRandomizer::new_limited(123, 0, 1000, Vec::new());

        for i in 0..100 {
            println!("{}: {}, {}", i, gen1.get(), gen2.get());
        }
    }
}
