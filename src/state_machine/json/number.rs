// will it start with a 0
// will it be a negative or positive
// fraction with a comma
// is integer or a irrational
// is non-decimal (octal, hex)
// is too large

use std::u64;

use super::randomization::*;
use crate::state_machine::{Automaton, AutomatonNode};

fn insert_char(seq: &mut String, to_insert: char, seed: u64) -> bool {
    match random_position(seq, seed as u32) {
        None => false,
        Some(insert_pos) => {
            seq.insert(insert_pos as usize, to_insert);
            true
        }
    }
}

#[allow(dead_code)]
static START_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=40 => Some(&REAL_NUMBER),
        41..=80 => Some(&NATURAL_NUMBER),
        // 81..=90 => Some(&super::null::START_NULL),
        91..=95 => Some(&HEX_NUMBER),
        _ => Some(&OCTAL_NUMBER),
    },
    transformation: |seed, _| random_digit_string(seed),
};

static REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=10 => Some(&SCI_NOTATION_REAL_NUMBER),
        _ => Some(&SIGNED_NUMBER),
    },
    transformation: |seed, num| match seed % 100 {
        0..=90 => {
            let mut to_return = String::from(num);
            insert_char(&mut to_return, '.', seed as u64);
            to_return
        }
        91..=100 => {
            let mut to_return = String::from(num);
            insert_char(&mut to_return, ',', seed as u64);
            to_return
        }
        _ => panic!("Invalid seed"),
    },
};

static NATURAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| Some(&SIGNED_NUMBER),
    transformation: super::IDENTITY,
};

static HEX_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |seed, _| format!("0x{}", random_digit_string(seed)),
};

static OCTAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |seed, _| format!("0{}", random_digit_string(seed)),
};

static SCI_NOTATION_REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| Some(&SIGNED_NUMBER),
    transformation: |seed, num| {
        let e = if random_bool(seed) { 'E' } else { 'e' };
        let sign = if random_bool(seed) { '+' } else { '-' };
        let exponent = random_digit_string(seed);
        format!("{}{}{}{}", num, e, sign, exponent)
    },
};

static SIGNED_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |seed, num| {
        if seed % 2 == 0 {
            format!("-{}", num)
        } else {
            num
        }
    },
};

#[allow(dead_code)]
pub static NUMBER_AUTOMATON: Automaton<String> = Automaton::<String> {
    initial_node: &START_NUMBER,
};
#[cfg(test)]
mod tests {

    // there's a shellcode injection case
    // there's int overflow case, etc.
    // #[test]
    // #[should_panic(expected = "Invalid seed")]
    // fn panic_when_seed_is_invalid() {
    //     super::StartBoolean.decide_next(123);
    // }

    #[test]
    fn try_number() {
        for _i in 1..20 {
            let res: String = super::NUMBER_AUTOMATON.traverse(String::from("asd"));
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn char_insertions_inserts() {
        let mut res_str = String::from("a");
        super::insert_char(&mut res_str, 'b', 1234);
        assert!(res_str == String::from("ba") || res_str == String::from("ab"));
    }

    #[test]
    fn char_insertions_inserts_at_the_right_place() {
        let mut res_str = String::from("helo");
        super::insert_char(&mut res_str, 'l', 1234);
        assert_eq!(res_str, String::from("hello"));
    }
}
