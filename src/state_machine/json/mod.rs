// use rand::{thread_rng, Rng};
// use bitvec::{Bits, store::BitStore};

pub mod boolean;
pub mod null;
pub mod number;
pub mod string;
use super::png::prandomizer::PRandomizer;
use super::png::skewed_prandomizer::SkewedPRandomizer;
use rand::{Rng, SeedableRng};
use rand_pcg::{Lcg64Xsh32, Pcg32};

pub const IDENTITY: fn(u32, String) -> String = |_, input| input;

pub mod randomization {
    use super::PRandomizer;
    use super::SkewedPRandomizer;

    pub fn char_flip_case(ch: char) -> char {
        if ch.is_uppercase() {
            ch.to_ascii_lowercase()
        } else {
            ch.to_ascii_uppercase()
        }
    }

    pub fn random_capitalization(seed: u32, to_transform: String) -> String {
        let mut transformed = String::new();
        let mut randomizer = PRandomizer::new(seed as u64);
        for ch in to_transform.chars() {
            let to_append: char = if randomizer.get() % 2 == 0 {
                ch
            } else {
                char_flip_case(ch)
            };
            transformed.push(to_append);
        }
        transformed
    }

    // less > 2^32
    pub fn random_digit_string(seed: u32) -> String {
        let mut randomizer =
            SkewedPRandomizer::new_limited(seed as u64, 0, 1_u32 << 20, vec![1, 1000]);
        randomizer.get().to_string()
    }

    pub fn random_digit_string_long(seed: u32) -> String {
        let mut randomizer = PRandomizer::new_limited(seed as u64, 1_u32 << 30, u32::MAX);
        randomizer.get().to_string().repeat(10)
    }

    pub fn random_position(word: &String, seed: u32) -> Option<u32> {
        if word.is_empty() {
            None
        } else {
            Some(seed % word.len() as u32)
        }
    }

    pub fn random_bool(seed: u32) -> bool {
        let mut randomizer = PRandomizer::new(seed as u64);
        randomizer.get() % 2 == 0
    }
}

#[cfg(test)]
mod common_json_tests {
    use super::randomization::*;

    #[test]
    fn flip_case_for_lowercase_char() {
        assert_eq!(char_flip_case('a'), 'A')
    }

    #[test]
    fn flip_case_for_uppercase_char() {
        assert_eq!(char_flip_case('A'), 'a')
    }

    #[test]
    fn random_capitalization_is_deterministic() {
        let res1 = random_capitalization(123321, String::from("this is my test"));
        let res2 = random_capitalization(123321, String::from("this is my test"));
        assert_eq!(res1, res2);
    }

    #[test]
    fn random_capitalization_depends_on_seed() {
        let res1 = random_capitalization(123322, String::from("this is my test"));
        let res2 = random_capitalization(123321, String::from("this is my test"));
        assert_ne!(res1, res2);
    }

    #[test]
    fn random_capitalization_can_randomize_uppercase_string() {
        let res1 = random_capitalization(123322, String::from("THIS IS MY TEST"));
        assert_ne!(res1, String::from("THIS IS MY TEST"));
    }

    #[test]
    fn random_position_for_empty_string_is_none() {
        let pos1 = random_position(&String::from(""), 0);
        let pos2 = random_position(&String::from(""), 123);
        let pos3 = random_position(&String::from(""), 123123);
        assert_eq!(None, pos1);
        assert_eq!(None, pos2);
        assert_eq!(None, pos3);
    }

    #[test]
    fn random_position_depends_on_seed() {
        let pos1 = random_position(&String::from("Some string"), 12345);
        let pos2 = random_position(&String::from("Some string"), 12346);
        assert_ne!(pos1, pos2);
    }

    #[test]
    fn random_position_is_always_in_strings_range() {
        let pos1 = random_position(&String::from("Bigger than seed"), 2);
        let pos2 = random_position(&String::from("Lesser than seed"), 123123);
        assert_eq!(pos1, Some(2));
        assert_eq!(pos2, Some(3));
    }
}

pub fn rand_num(seed: u64) -> Pcg32 {
    Pcg32::seed_from_u64(seed)
}
