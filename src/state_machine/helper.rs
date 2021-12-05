use super::{PRandomizer, Randomizer};

pub fn char_flip_case(ch: char) -> char {
    if ch.is_uppercase() {
        ch.to_ascii_lowercase()
    } else {
        ch.to_ascii_uppercase()
    }
}

pub fn random_capitalization(seed: u64, to_transform: String) -> String {
    let mut transformed = String::new();
    let mut randomizer = PRandomizer::new(seed);
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

pub fn random_digit_string(seed: u64) -> String {
    seed.to_string()
}

pub fn to_upper_case(s: String) -> String {
    s.to_ascii_uppercase()
}

pub fn to_capitalized(mut s: String) -> String {
    s[0..1].make_ascii_uppercase();
    s
}

pub fn to_random_case(s: String) -> String {
    random_capitalization(123, s)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
