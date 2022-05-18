use super::super::helper::*;
use crate::state_machine::{Automaton, AutomatonNode};

lazy_static! {
    static ref START_INDENTATION: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (1, &TABULATED_INDENTATION_2),
            (1, &TABULATED_INDENTATION_4),
            (1, &EXPANDED_INDENTATION),
            (1, &SHRINKED_INDENTATION),
            (4, &FINAL)
        ]);
    pub static ref TABULATED_INDENTATION_2: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|seed, spaces| { replace_random_occurrence(spaces, "  ", "\t", seed) })
        .set_cycle(1);
    pub static ref TABULATED_INDENTATION_4: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|seed, spaces| { replace_random_occurrence(spaces, "    ", "\t", seed) })
        .set_cycle(1);
    pub static ref EXPANDED_INDENTATION: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, mut spaces| {
            spaces.push(' ');
            spaces
        })
        .set_cycle(1);
    pub static ref SHRINKED_INDENTATION: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, mut spaces| {
            spaces.pop();
            spaces
        })
        .set_cycle(1);
    pub static ref INDENTATION_AUTOMATON: Automaton<String> = Automaton::<String> {
        initial_node: &START_INDENTATION,
        generator: |_| String::from("  "),
    };
}

#[cfg(test)]
mod tests {
    use super::INDENTATION_AUTOMATON;
    use itertools::Itertools;

    lazy_static! {
        // sorted list of a 1000 fuzzed indentation values
        static ref TEST_FUZZ_VALUES: Vec<String> = (1..1000)
            .map(|i| INDENTATION_AUTOMATON.generate(i))
            .sorted()
            .collect();
    }

    #[test]
    fn indentation_automaton_is_seedable() {
        assert_ne!(TEST_FUZZ_VALUES.last(), TEST_FUZZ_VALUES.first());
    }

    #[test]
    fn result_is_diverse_enough() {
        let unique_values = TEST_FUZZ_VALUES.iter().unique().count();
        assert!(unique_values > 5);
    }

    #[test]
    fn try_indentation2() {
        for i in 1..20 {
            let res: String = super::INDENTATION_AUTOMATON.generate(i);
            println!("Res is: {}", res);
        }
    }
}
