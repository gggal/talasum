use super::randomization::*;
use crate::state_machine::{Automaton, AutomatonNode};

#[allow(dead_code)]
static START_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=40 => Some(&VALID_NULL),
        41..=60 => Some(&NIL_NULL),
        61..=80 => Some(&NONE_NULL),
        _ => Some(&ZERO_NULL),
    },
    transformation: super::IDENTITY,
};

static VALID_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| Some(&CASED_NULL),
    transformation: |_, _| String::from("null"),
};

static NIL_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| Some(&CASED_NULL),
    transformation: |_, _| String::from("nil"),
};

static NONE_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| Some(&CASED_NULL),
    transformation: |_, _| String::from("none"),
};

static ZERO_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |_, _| String::from("0"),
};

static CASED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |seed, text| match seed % 100 {
        0..=10 => to_upper_case(seed, text),
        11..=20 => to_capitalized(seed, text),
        21..=30 => to_random_case(seed, text),
        _ => text,
    },
};

#[allow(dead_code)]
pub static NULL_AUTOMATON: Automaton<String> = Automaton::<String> {
    initial_node: &START_NULL,
};

#[cfg(test)]
mod tests {
    use super::NULL_AUTOMATON;

    #[test]
    fn try_null() {
        for _i in 1..20 {
            let res: String = NULL_AUTOMATON.traverse(String::from("asd"));
            println!("Res is: {}", res);
        }
    }
}
