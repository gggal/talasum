use std::char;

use super::{AutomatonNode, PRandomizer, Randomizer};

/// The Identity function
pub const IDENTITY: fn(u64, String) -> String = |_, input| input;

lazy_static! {
    /// A trivial final state without a transformation function
    pub static ref FINAL: AutomatonNode<String> = AutomatonNode::<String>::new();
}

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

pub fn to_upper_case(_: u64, s: String) -> String {
    s.to_ascii_uppercase()
}

pub fn to_capitalized(_: u64, mut s: String) -> String {
    s[0..1].make_ascii_uppercase();
    s
}

pub fn to_random_case(seed: u64, s: String) -> String {
    random_capitalization(seed, s)
}

/// Pick a position in a string based on its content. If the
/// string is empty, None is returned.
pub fn random_position_in_string(seed: u64, s: &str) -> Option<usize> {
    if !s.is_empty() {
        seed.checked_rem_euclid(s.len() as u64).map(|mut n| {
            while !s.is_char_boundary(n as usize) {
                n += 1;
            }
            n as usize
        })
    } else {
        None
    }
}

fn is_surrogate(num: u32) -> bool {
    (0xD800..=0xDFFF).contains(&num)
}

// Pick a character based on input u64 value
fn get_unicode_char(n: u64) -> char {
    // get number < 2 ^ 20 and then remove surrogate
    let mut to_be_char = (n % (1 << 20) as u64) as u32;
    if is_surrogate(to_be_char) {
        to_be_char += 2048;
    }
    char::from_u32(to_be_char).expect("Number not valid unicode character!")
}

// Pick an control character based on input u64 value
fn get_control_char(n: u64) -> String {
    let mut to_be_char = (n % (1 << 6) as u64) as u32;
    // in case of C1 control
    if to_be_char > 0x1F {
        to_be_char += 0x80
    }
    format!("\\u{:04x}", to_be_char)
}

// Pick a surrogate codepoint based on input u64 value
fn get_surrogate(n: u64) -> String {
    let num_to_insert = (n % 2048_u64) as u32;
    format!("\\u{:04x}", num_to_insert + 0xD800)
}

fn get_surrogate_pair(n: u64) -> String {
    let first_num = n << 32;
    let sec_num = n >> 32;

    let high_surrogate = (first_num % 1024_u64) as u32 + 0xD800;
    let low_surrogate = (sec_num % 1024_u64) as u32 + 0xDC00;
    format!("\\u{:04x}\\u{:04x}", high_surrogate, low_surrogate)
}

/// Insert a char in a string based on input value
pub fn insert_random_char_in_string(seed: u64, s: &str) -> String {
    insert_string_in_string(seed, s, &get_unicode_char(seed).to_string())
}

/// Insert an unescaped control char in a string based on input value
pub fn insert_random_unescaped_control_char(seed: u64, s: &str) -> String {
    insert_string_in_string(seed, s, &get_control_char(seed))
}
/// Insert a string somewhere in a string
pub fn insert_string_in_string(seed: u64, s: &str, to_insert: &str) -> String {
    match random_position_in_string(seed, s) {
        Some(pos) => {
            let mut to_return = s.to_string();
            to_return.insert_str(pos, to_insert);
            to_return
        }
        None => to_insert.to_string(),
    }
}

pub fn insert_random_surrogate_in_string(seed: u64, s: &str) -> String {
    insert_string_in_string(seed, s, &get_surrogate(seed))
}

pub fn insert_random_surrogate_pair_in_string(seed: u64, s: &str) -> String {
    insert_string_in_string(seed, s, &get_surrogate_pair(seed))
}

pub fn insert_random_encoded_char_in_string(seed: u64, s: &str) -> String {
    insert_string_in_string(
        seed,
        s,
        &format!("\\u{:04x}", get_unicode_char(seed) as u32),
    )
}

/// Picks a random character from a string and puts it at
/// random position in `s`.
pub fn insert_random_char_from_range_in_string(seed: u64, s: &str, chars: &str) -> String {
    insert_string_in_string(seed, s, &pick_random_char(seed, chars))
}

pub fn pick_random_char(seed: u64, s: &str) -> String {
    match random_position_in_string(seed, s) {
        None => String::new(),
        Some(pick) => String::from(
            s.get(pick..pick + 1)
                .expect("Picked position was out of bounds."),
        ),
    }
}

/// Replaces a random occurrence of substring with another substring
pub fn replace_random_occurrence(
    mut original: String,
    to_replace: &str,
    replace_with: &str,
    seed: u64,
) -> String {
    // TODO wouldn't work for non-utf8
    let replace_possibilities: Vec<(usize, &str)> = original.rmatch_indices(to_replace).collect();
    if replace_possibilities.len() != 0 {
        let (choice, _) =
            replace_possibilities[(seed % replace_possibilities.len() as u64) as usize];
        let to_replace_length = to_replace.len();
        original.replace_range(choice..choice + (to_replace_length), replace_with);
    }
    original
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

    #[test]
    fn capitalized_works() {
        assert_eq!(
            super::to_capitalized(0, String::from("word")),
            String::from("Word")
        );
    }

    #[test]
    fn to_capitalized_does_not_depend_on_seed() {
        assert_eq!(
            to_capitalized(100, String::from("word")),
            to_capitalized(0, String::from("word"))
        );
    }

    #[test]
    fn to_upper_case_works() {
        assert_eq!(String::from("TEST"), to_upper_case(0, String::from("tesT")));
    }

    #[test]
    fn to_upper_case_does_not_depend_on_seed() {
        assert_eq!(
            to_upper_case(100, String::from("tesT")),
            to_upper_case(0, String::from("tesT"))
        );
    }

    #[test]
    fn to_random_case_works() {
        let input = String::from("test");
        assert_ne!(input, to_random_case(0, input.clone()));
    }

    #[test]
    fn to_random_case_does_not_depend_on_seed() {
        let input = String::from("test");
        assert_ne!(
            to_random_case(100, input.clone()),
            to_random_case(0, input.clone())
        );
    }

    #[test]
    fn picking_position_in_empty_string_fails() {
        assert!(random_position_in_string(0, &String::new()).is_none());
    }

    #[test]
    fn picking_position_among_ascii_chars_works() {
        assert_eq!(
            random_position_in_string(0, &String::from("asd")).unwrap(),
            0
        );
    }

    #[test]
    fn picking_position_among_2_byte_chars_works() {
        assert_eq!(
            random_position_in_string(1, &String::from("aфd")).unwrap(),
            1
        );
        assert_eq!(
            random_position_in_string(2, &String::from("aфd")).unwrap(),
            3
        );
    }

    #[test]
    fn pick_character_for_large_number() {
        assert_eq!(get_unicode_char((1 << 20) as u64 + 97), 'a');
    }

    #[test]
    fn avoid_surrogates_while_picking_character() {
        assert_eq!(
            get_unicode_char(0xD801),
            std::char::from_u32(0xD801 + 2048).expect("Not a valid surrogate char")
        );
    }

    #[test]
    fn pick_get_char_returns_correct_value() {
        assert_eq!(get_unicode_char(97), 'a');
    }

    #[test]
    fn surrogate_codepoint_is_properly_formatted() {
        assert_eq!(get_surrogate(0), "\\ud800");
    }

    #[test]
    fn pick_surrogate_codepoint_from_larger_number() {
        assert_eq!(get_surrogate(1 << 20), "\\ud800");
    }

    #[test]
    fn replace_single_occurrence_in_string() {
        assert_eq!(
            replace_random_occurrence(String::from("asf"), "a", "b", 1),
            "bsf"
        );
    }

    #[test]
    fn replace_non_first_occurrence_in_string() {
        assert_eq!(
            replace_random_occurrence(String::from("asfasd"), "a", "b", 2),
            "asfbsd"
        );
    }

    #[test]
    fn replace_occurrence_not_in_string() {
        assert_eq!(
            replace_random_occurrence(String::from("asf"), "c", "b", 1),
            "asf"
        );
    }
}
