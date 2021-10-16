use super::randomization::*;
use crate::state_machine::{Automaton, AutomatonNode};

#[allow(dead_code)]
static START_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=40 => Some(&CASED_NULL),
        41..=60 => Some(&NIL_NULL),
        61..=80 => Some(&NONE_NULL),
        _ => Some(&ZERO_NULL),
    },
    transformation: super::IDENTITY,
};

static NIL_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| Some(&CASED_NULL),
    transformation: |_| String::from("nil"),
};

static NONE_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| Some(&CASED_NULL),
    transformation: |_| String::from("none"),
};

static ZERO_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |_| String::from("0"),
};

static CASED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed| match seed % 100 {
        0..=10 => Some(&UPPER_CASED_NULL),
        11..=20 => Some(&RANDOM_CASED_NULL),
        21..=30 => Some(&CAPITALIZED_NULL),
        _ => None,
    },
    transformation: super::IDENTITY,
};

static UPPER_CASED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |text| to_upper_case(text)
};

static RANDOM_CASED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |text| to_random_case(text)
};

static CAPITALIZED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |text| to_capitalized(text)
};

#[allow(dead_code)]
pub static NULL_AUTOMATON: Automaton<String> = Automaton::<String> {
    initial_node: &START_NULL,
    generator: |_| String::from("null"),
};

#[cfg(test)]
mod tests {
    use super::NULL_AUTOMATON;

    #[test]
    fn try_null() {
        for _i in 1..20 {
            let res: String = NULL_AUTOMATON.traverse(String::from("null"));
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn try_null1() {
        for _i in 1..20 {
            let res: String = NULL_AUTOMATON.generate();
            println!("Res is: {}", res);
        }
    }
}
