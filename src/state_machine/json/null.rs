use super::super::helper::*;
use super::super::weights::*;
use crate::state_machine::{Automaton, AutomatonNode};

lazy_static! {
    static ref CASED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![
            (2, Some(&UPPER_CASED_NULL)),
            (1, Some(&RANDOM_CASED_NULL)),
            (2, Some(&CAPITALIZED_NULL))
        ]),
        transformation: IDENTITY,
    };
    pub static ref START_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![
            (1, Some(&CASED_NULL)),
            (1, Some(&NIL_NULL)),
            (1, Some(&NONE_NULL)),
            (3, Some(&ZERO_NULL)),
            (2, Some(&EMPTY_NULL))
        ]),
        transformation: IDENTITY,
    };
    static ref NIL_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: Box::new(|_| Some(&CASED_NULL)),
        transformation: |_| String::from("nil"),
    };
    static ref NONE_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: Box::new(|_| Some(&CASED_NULL)),
        transformation: |_| String::from("none"),
    };
    static ref ZERO_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: Box::new(|_| None),
        transformation: |_| String::from("0"),
    };
    static ref EMPTY_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: Box::new(|_| None),
        transformation: |_| String::from(""),
    };
    static ref UPPER_CASED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: Box::new(|_| None),
        transformation: |text| to_upper_case(text)
    };
    static ref RANDOM_CASED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: Box::new(|_| None),
        transformation: |text| to_random_case(text)
    };
    static ref CAPITALIZED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: Box::new(|_| None),
        transformation: |text| to_capitalized(text)
    };
    pub static ref NULL_AUTOMATON: Automaton<String> = Automaton::<String> {
        initial_node: &START_NULL,
        generator: |_| String::from("null"),
    };
}

#[cfg(test)]
mod tests {
    use super::NULL_AUTOMATON;
    use itertools::Itertools;

    lazy_static! {
        // sorted list of a 1000 fuzzed null values
        static ref TEST_FUZZ_VALUES: Vec<String> = (1..1000)
            .map(|i| NULL_AUTOMATON.generate(i))
            .sorted()
            .collect();
    }

    #[test]
    fn null_automaton_is_seedable() {
        assert_ne!(TEST_FUZZ_VALUES.last(), TEST_FUZZ_VALUES.first());
    }

    #[test]
    fn result_is_diverse_enough() {
        let unique_values = TEST_FUZZ_VALUES.iter().unique().count();
        assert!(unique_values > 5);
    }

    #[test]
    fn try_null2() {
        for i in 1..20 {
            let res: String = super::NULL_AUTOMATON.generate(i);
            println!("Res is: {}", res);
        }
    }
}
