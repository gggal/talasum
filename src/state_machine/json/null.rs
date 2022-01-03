use super::super::helper::*;
use crate::state_machine::{json::whitespace::START_WS, Automaton, AutomatonNode};

lazy_static! {
    static ref CASED_NULL: AutomatonNode<String> = AutomatonNode::<String>::new().set_edges(vec![
        (2, &UPPER_CASED_NULL),
        (1, &RANDOM_CASED_NULL),
        (2, &CAPITALIZED_NULL)
    ]);
    pub static ref START_NULL: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (1, &CASED_NULL),
            (1, &NIL_NULL),
            (1, &NONE_NULL),
            (3, &ZERO_NULL),
            (2, &EMPTY_NULL)
        ]);
    static ref NIL_NULL: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edge(&CASED_NULL)
        .set_func(|_| String::from("nil"));
    static ref NONE_NULL: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edge(&CASED_NULL)
        .set_func(|_| String::from("none"));
    static ref ZERO_NULL: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_| String::from("0"),)
        .set_edge(&START_WS);
    static ref EMPTY_NULL: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_| String::from(""),)
        .set_edge(&START_WS);
    static ref UPPER_CASED_NULL: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(to_upper_case)
        .set_edge(&START_WS);
    static ref RANDOM_CASED_NULL: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(to_random_case)
        .set_edge(&START_WS);
    static ref CAPITALIZED_NULL: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(to_capitalized)
        .set_edge(&START_WS);
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
