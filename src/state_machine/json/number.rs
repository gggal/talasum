// will it start with a 0
// will it be a negative or positive
// fraction with a comma
// is integer or a irrational
// is non-decimal (octal, hex)
// is too large

use std::u64;

use super::randomization::*;
use crate::state_machine::{Automaton, AutomatonEdge, AutomatonState};

#[derive(Default, Debug)]
pub struct NumberAutomaton {
    val: String,
}

impl NumberAutomaton {
    fn get_start_state() -> Box<dyn AutomatonState<String>> {
        return Box::new(StartNumber);
    }
}

impl Automaton<String> for NumberAutomaton {
    fn init_value(&self) -> String {
        self.val.clone()
    }

    fn init_state(&self) -> Box<dyn AutomatonState<String>> {
        Self::get_start_state()
    }
}

struct StartNumber;
impl AutomatonState<String> for StartNumber {
    fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
        match seed % 100 {
            0..=40 => Some((Box::new(RealNumber), |seed, _| random_digit_string(seed))),
            41..=80 => Some((Box::new(NaturalNumber), |seed, _| random_digit_string(seed))),
            81..=90 => Some((Box::new(super::null::StartNull), super::IDENTITY)),
            91..=95 => Some((Box::new(Final), |seed, _| {
                format!("0x{}", random_digit_string(seed))
            })),
            96..=100 => Some((Box::new(Final), |seed, _| {
                format!("0{}", random_digit_string(seed))
            })),
            _ => panic!("Invalid seed"),
        }
    }
}

struct Final;
impl AutomatonState<String> for Final {
    fn decide_next(&self, _seed: u32) -> Option<AutomatonEdge<String>> {
        None
    }
}

struct RealNumber;
impl AutomatonState<String> for RealNumber {
    fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
        match seed % 100 {
            0..=90 => Some((Box::new(ScientificNotationRealNumber), |seed, num| {
                let mut to_return = String::from(num);
                insert_char(&mut to_return, '.', seed as u64);
                to_return
            })),
            91..=100 => Some((Box::new(ScientificNotationRealNumber), |seed, num| {
                let mut to_return = String::from(num);
                insert_char(&mut to_return, ',', seed as u64);
                to_return
            })),
            _ => panic!("Invalid seed"),
        }
    }
}

struct NaturalNumber;
impl AutomatonState<String> for NaturalNumber {
    fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
        match seed % 100 {
            0..=90 => Some((Box::new(SignedInt), super::IDENTITY)),
            91..=100 => Some((Box::new(SignedInt), |seed, _| {
                random_digit_string_long(seed)
            })),
            _ => panic!("Invalid seed"),
        }
    }
}

struct ScientificNotationRealNumber;
impl AutomatonState<String> for ScientificNotationRealNumber {
    fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
        match seed % 100 {
            0..=95 => Some((Box::new(SignedInt), super::IDENTITY)),
            96..=100 => Some((Box::new(SignedInt), |seed, num| {
                let e = if random_bool(seed) { 'E' } else { 'e' };
                let sign = if random_bool(seed) { '+' } else { '-' };
                let exponent = random_digit_string(seed);
                format!("{}{}{}{}", num, e, sign, exponent)
            })),
            _ => panic!("Invalid seed"),
        }
    }
}

struct SignedInt;
impl AutomatonState<String> for SignedInt {
    fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
        match seed % 100 {
            0..=50 => Some((Box::new(Final), super::IDENTITY)),
            51..=100 => Some((Box::new(Final), |_, num| format!("-{}", num))),
            _ => panic!("Invalid seed"),
        }
    }
}

fn insert_char(seq: &mut String, to_insert: char, seed: u64) -> bool {
    match random_position(seq, seed as u32) {
        None => false,
        Some(insert_pos) => {
            seq.insert(insert_pos as usize, to_insert);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::state_machine::Automaton;
    use crate::state_machine::AutomatonState;

    // there's a shellcode injection case
    // there's int overflow case, etc.
    // #[test]
    // #[should_panic(expected = "Invalid seed")]
    // fn panic_when_seed_is_invalid() {
    //     super::StartBoolean.decide_next(123);
    // }

    #[test]
    fn try_number() {
        let mut my_machine: super::NumberAutomaton = super::NumberAutomaton::default();
        for _i in 1..20 {
            let res = my_machine.traverse();
            println!("Res is: {}", res);
        }
        super::StartNumber.decide_next(123);
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
