use super::super::weights::*;
use super::randomization::*;
use crate::state_machine::{Automaton, Automaton1, AutomatonNode, AutomatonNode1};

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
    transformation: |text| to_upper_case(text),
};

static RANDOM_CASED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |text| to_random_case(text),
};

static CAPITALIZED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |text| to_capitalized(text),
};

// CAN WE MERGE TRANSITIONS AND TRANSFORMATIONS
// Transformation may be a |string| -> string function
// Transition may be a function, created by the extremize_and_choose
// which would return a func rather than result; the constructor
// will be executed once, thus the func generation will be executed once

// #[allow(dead_code)]
lazy_static! {
    static ref CASED_NULL1: AutomatonNode1<String> = AutomatonNode1::<String> {
        transition: TransitionChoice::new(vec![
            (1, Some(&UPPER_CASED_NULL1)),
            (1, Some(&RANDOM_CASED_NULL1)),
            (1, Some(&CAPITALIZED_NULL1)),
            // WeightedTransition{weight: 7.0, value: None},
        ], 100).choose(),
        transformation: super::IDENTITY,
    };


    static ref START_NULL1: AutomatonNode1<String> = AutomatonNode1::<String> {
        transition: TransitionChoice::new(vec![
            (4, Some(&CASED_NULL1)),
            (2, Some(&NIL_NULL1)),
            (2, Some(&NONE_NULL1)),
            (2, Some(&ZERO_NULL1)),
        ], 100).choose(),
        transformation: super::IDENTITY,
    };


    static ref START_NULL2: AutomatonNode1<String> = AutomatonNode1::<String> {
        transition: TransitionChoice::new(vec![
            (4, Some(&CASED_NULL1)),
            (2, Some(&NIL_NULL1)),
            (2, Some(&NONE_NULL1)),
            (2, Some(&ZERO_NULL1)),
        ], 50).choose(),
        transformation: super::IDENTITY,
    };

    static ref NIL_NULL1: AutomatonNode1<String> = AutomatonNode1::<String> {
        transition: Box::new(|_| Some(&CASED_NULL1)),
        transformation: |_| String::from("nil"),
    };

    static ref NONE_NULL1: AutomatonNode1<String> = AutomatonNode1::<String> {
        transition: Box::new(|_| Some(&CASED_NULL1)),
        transformation: |_| String::from("none"),
    };

    static ref ZERO_NULL1: AutomatonNode1<String> = AutomatonNode1::<String> {
        transition: Box::new(|_| None),
        transformation: |_| String::from("0"),
    };

    static ref UPPER_CASED_NULL1: AutomatonNode1<String> = AutomatonNode1::<String> {
        transition: Box::new(|_| None),
        transformation: |text| to_upper_case(text)
    };

    static ref RANDOM_CASED_NULL1: AutomatonNode1<String> = AutomatonNode1::<String> {
        transition: Box::new(|_| None),
        transformation: |text| to_random_case(text)
    };

    static ref CAPITALIZED_NULL1: AutomatonNode1<String> = AutomatonNode1::<String> {
        transition: Box::new(|_| None),
        transformation: |text| to_capitalized(text)
    };

    #[allow(dead_code)]
    pub static ref NULL_AUTOMATON1: Automaton1<String> = Automaton1::<String> {
        initial_node: &START_NULL1,
        generator: |_| String::from("null"),
    };

}

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

    #[test]
    fn try_null2() {
        for _i in 1..20 {
            let res: String = super::NULL_AUTOMATON1.generate();
            println!("Res is: {}", res);
        }
    }
}
