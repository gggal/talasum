use super::{randomization::*, IDENTITY};
use crate::state_machine::{Automaton, AutomatonNode};

#[allow(dead_code)]
static START_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=40 => Some(&REAL_NUMBER),
        41..=80 => Some(&NATURAL_NUMBER),
        // 81..=90 => Some(&super::null::START_NULL),
        91..=95 => Some(&HEX_NUMBER),
        _ => Some(&OCTAL_NUMBER),
    },
    transformation: IDENTITY,
};

static REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=10 => Some(&SCI_NOTATION_REAL_NUMBER),
        11..=45 => Some(&SIGNED_NUMBER),
        46..=60 => Some(&DECIMAL_COMMA_REAL_NUMBER),
        _ => None,
    },
    transformation: |num| {
        let num1 = num.parse::<u32>().unwrap();
        let delim = num1 % 10 + 1;
        (num1 / (100 * delim)).to_string()
    },
};

static DECIMAL_COMMA_REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=50 => Some(&SIGNED_NUMBER),
        _ => None,
    },
    transformation: |num| str::replace(&num, ".", ","),
};

static NATURAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=50 => Some(&SIGNED_NUMBER),
        _ => None,
    },
    transformation: super::IDENTITY,
};

static HEX_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    // transformation: |seed, _| format!("0x{}", random_digit_string(seed)),
    transformation: |input| format!("{:#01x}", input.parse::<u32>().unwrap()),
};

static OCTAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |input| format!("0{:o}", input.parse::<u32>().unwrap()),
};

static SCI_NOTATION_REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=50 => Some(&SIGNED_NUMBER),
        _ => None,
    },transformation: |num| {
        format!("{:+e}", num.parse::<u32>().unwrap())
    },
};

static SIGNED_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |num| format!("-{}", num),
};

#[allow(dead_code)]
pub static NUMBER_AUTOMATON: Automaton<String> = Automaton::<String> {
    initial_node: &START_NUMBER,
    generator: |seed| random_digit_string(seed),
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
            let res: String = super::NUMBER_AUTOMATON.traverse(String::from("1"));
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn try_number1() {
        for _i in 1..20 {
            let res: String = super::NUMBER_AUTOMATON.generate();
            println!("Res is: {}", res);
        }
    }
}
