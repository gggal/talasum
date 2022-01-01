use super::super::helper::*;
use crate::state_machine::{Automaton, AutomatonNode};

lazy_static! {
    static ref START_BOOLEAN: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![(2, &FINAL), (1, &REVERSE_BOOLEAN)]);
    static ref REVERSE_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![(1, &CASED_BOOLEAN), (1, &NUMERICAL_BOOLEAN)])
        .set_func(|input| {
            if input == "true" {
                String::from("false")
            } else {
                String::from("true")
            }
        });
    static ref NUMERICAL_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![(1, &QUOTED_BOOLEAN), (3, &CASED_BOOLEAN)])
        .set_func(|input| {
            if input == "true" {
                String::from("1")
            } else {
                String::from("0")
            }
        });
    static ref QUOTED_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edge(&CASED_BOOLEAN)
        .set_func(|text| format!("\"{}\"", text));
    static ref CASED_BOOLEAN: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (1, &UPPER_CASED_BOOLEAN),
            (1, &RANDOM_CASED_BOOLEAN),
            (2, &CAPITALIZED_BOOLEAN),
            (2, &FINAL)
        ]);
    static ref UPPER_CASED_BOOLEAN: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(to_upper_case);
    static ref RANDOM_CASED_BOOLEAN: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(to_random_case);
    static ref CAPITALIZED_BOOLEAN: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(to_capitalized);
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
    use itertools::Itertools;

    lazy_static! {
        // sorted list of a 1000 fuzzed bool values
        static ref TEST_FUZZ_VALUES: Vec<String> = (1..1000)
            .map(|i| BOOL_AUTOMATON.generate(i))
            .sorted()
            .collect();
    }

    #[test]
    fn bool_automaton_is_seedable() {
        assert_ne!(TEST_FUZZ_VALUES.last(), TEST_FUZZ_VALUES.first());
    }

    #[test]
    fn true_and_false_results_are_equally_likely() {
        let trues = TEST_FUZZ_VALUES
            .iter()
            .filter(|fuzzed| "true".eq_ignore_ascii_case(fuzzed))
            .count();
        let falses = TEST_FUZZ_VALUES
            .iter()
            .filter(|fuzzed| "false".eq_ignore_ascii_case(fuzzed))
            .count();
        let delta = 20_usize;
        assert!(trues < falses + delta);
        assert!(trues > falses - delta);
    }

    #[test]
    fn result_is_diverse_enough() {
        let unique_values = TEST_FUZZ_VALUES.iter().unique().count();
        assert!(unique_values > 5);
    }

    #[test]
    fn try_bool1() {
        for i in 1..20 {
            let res: String = BOOL_AUTOMATON.generate(i);
            println!("Res is: {}", res);
        }
    }
}
