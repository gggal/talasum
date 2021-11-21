use super::super::weights::*;
use super::randomization::*;
use crate::state_machine::{Automaton, AutomatonNode};

lazy_static! {
    static ref START_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![(4, None), (1, Some(&REVERSE_BOOLEAN))],
        transformation: super::IDENTITY,
    };
    static ref REVERSE_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![(1, Some(&CASED_BOOLEAN)), (1, Some(&NUMERICAL_BOOLEAN))],
        transformation: |input| {
            if input == "true" {
                String::from("false")
            } else {
                String::from("true")
            }
        },
    };
    static ref NUMERICAL_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![(1, Some(&QUOTED_BOOLEAN)), (4, Some(&CASED_BOOLEAN))],
        transformation: |input| {
            if input == "true" {
                String::from("1")
            } else {
                String::from("0")
            }
        },
    };
    static ref QUOTED_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![(1, Some(&CASED_BOOLEAN))],
        transformation: |text| format!("\"{}\"", text),
    };
    static ref CASED_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![
            (1, Some(&UPPER_CASED_BOOLEAN)),
            (1, Some(&RANDOM_CASED_BOOLEAN)),
            (1, Some(&CAPITALIZED_BOOLEAN)),
            (7, None)
        ],
        transformation: super::IDENTITY,
    };
    static ref UPPER_CASED_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![(1, None)],
        transformation: |text| to_upper_case(text),
    };
    static ref RANDOM_CASED_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![(1, None)],
        transformation: |text| to_random_case(text),
    };
    static ref CAPITALIZED_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![(1, None)],
        transformation: |text| to_capitalized(text),
    };
    pub static ref BOOL_AUTOMATON: Automaton<String> = Automaton::<String> {
        initial_node: &START_BOOLEAN,
        generator: |seed| {
            if seed % 2 == 0 {
                String::from("true")
            } else {
                String::from("false")
            }
        },
    };
}

#[cfg(test)]
mod tests {
    use super::BOOL_AUTOMATON;

    #[test]
    fn try_bool() {
        for _i in 1..20 {
            let res: String = BOOL_AUTOMATON.traverse(String::from("false"), 123);
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn try_bool1() {
        for _i in 1..20 {
            let res: String = BOOL_AUTOMATON.generate(123);
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn capitalized_works() {
        assert_eq!(
            super::to_capitalized(String::from("word")),
            String::from("Word")
        );
    }
}
