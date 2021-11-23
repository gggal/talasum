use super::super::weights::*;
use super::randomization::*;
use crate::state_machine::{Automaton, AutomatonNode};

lazy_static! {
    static ref CASED_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![
            (1, Some(&UPPER_CASED_NULL)),
            (1, Some(&RANDOM_CASED_NULL)),
            (1, Some(&CAPITALIZED_NULL))
        ],
        transformation: super::IDENTITY,
    };

    static ref START_NULL: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose![
            (4, Some(&CASED_NULL)),
            (2, Some(&NIL_NULL)),
            (2, Some(&NONE_NULL)),
            (2, Some(&ZERO_NULL))
        ],
        transformation: super::IDENTITY,
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

    #[allow(dead_code)]
    pub static ref NULL_AUTOMATON: Automaton<String> = Automaton::<String> {
        initial_node: &START_NULL,
        generator: |_| String::from("null"),
    };

}

#[cfg(test)]
mod tests {
    use super::NULL_AUTOMATON;

    #[test]
    fn try_null() {
        for _i in 1..20 {
            let res: String = NULL_AUTOMATON.traverse(String::from("null"), 123);
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn try_null2() {
        for _i in 1..20 {
            let res: String = super::NULL_AUTOMATON.generate(123);
            println!("Res is: {}", res);
        }
    }
}
