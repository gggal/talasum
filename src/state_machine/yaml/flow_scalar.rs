use super::super::helper::*;
use crate::state_machine::{json::string::STRING_AUTOMATON, Automaton, AutomatonNode};

lazy_static! {
    static ref START_FLOW_SCALAR: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (1, &MULTI_LINE_QUOTED_SCALAR),
            (1, &SINGLE_QUOTED_SCALAR),
            (1, &UNQUOTED_SCALAR),
            (5, &FINAL)
        ]);
    static ref MULTI_LINE_QUOTED_SCALAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|seed, text| insert_string_in_string(seed, &text, "\n"))
        .set_cycle(1);
    static ref SINGLE_QUOTED_SCALAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| { text.replace('\"', "'") })
        .set_edges(vec![
            (1, &ESCAPED_QUOTES_SINGLE_QUOTED_SCALAR),
            (5, &MULTI_LINE_QUOTED_SCALAR),
            (10, &FINAL)
        ]);
    static ref ESCAPED_QUOTES_SINGLE_QUOTED_SCALAR: AutomatonNode<String> =
        AutomatonNode::<String>::new()
            .set_func(|seed, text| insert_string_in_string(seed, &text, "''"))
            .set_cycle(1);
    static ref UNQUOTED_SCALAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_, text| { text.replace('\"', "") })
        .set_edges(vec![
            (1, &INVALID_UNQUOTED_SCALAR),
            (5, &VALID_UNQUOTED_SCALAR),
            (5, &FINAL)
        ]);
    static ref INVALID_UNQUOTED_SCALAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![
            (1, &LEADING_INDICATOR_SCALAR),
            (1, &FORBIDDEN_SUBSTR_SCALAR),
            (1, &INVALID_FLOW_COLLECTION_SCALAR),
            (1, &INVALID_IMPLICIT_KEY_SCALAR),
            (5, &FINAL)
        ]);
    static ref LEADING_INDICATOR_SCALAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|seed, text| format!("{}{}", pick_random_char(seed, "#[],-?:{{}}&*!|>\"'%@"), text)).set_cycle(2);
    static ref FORBIDDEN_SUBSTR_SCALAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|seed, text| {
            if seed % 2 == 0 {
                insert_string_in_string(seed, &text, ": ")
            } else {
                insert_string_in_string(seed, &text, " #")
            }
        }).set_cycle(2);
    static ref INVALID_FLOW_COLLECTION_SCALAR: AutomatonNode<String> = AutomatonNode::<String>::new()
    .set_func(|seed, text| insert_random_char_from_range_in_string(seed, &text, "[],{{}}"));
    static ref INVALID_IMPLICIT_KEY_SCALAR: AutomatonNode<String> = AutomatonNode::<String>::new()
    .set_func(|seed, text| insert_string_in_string(seed, &text, "\n"));

    // The result should be diversed enough already
    static ref VALID_UNQUOTED_SCALAR: AutomatonNode<String> = AutomatonNode::<String>::new();

    pub static ref FLOW_SCALAR_AUTOMATON: Automaton<String> = Automaton::<String> {
        initial_node: &START_FLOW_SCALAR,
        generator: |seed| { STRING_AUTOMATON.generate(seed) },
    };

}

#[cfg(test)]
mod tests {
    use crate::randomness::{PRandomizer, Randomizer};

    use super::FLOW_SCALAR_AUTOMATON;
    use itertools::Itertools;

    lazy_static! {
        // sorted list of a 1000 fuzzed number values
        static ref TEST_FUZZ_VALUES: Vec<String> = (1..1000)
            .map(|i| FLOW_SCALAR_AUTOMATON.generate(i))
            .sorted()
            .collect();
    }

    #[test]
    fn string_automaton_is_seedable() {
        assert_ne!(TEST_FUZZ_VALUES.last(), TEST_FUZZ_VALUES.first());
    }

    #[test]
    fn result_is_diverse_enough() {
        let unique_values = TEST_FUZZ_VALUES.iter().unique().count();
        assert!(unique_values > 15);
    }

    #[test]
    fn try_string() {
        for i in 1..20 {
            let res: String = super::FLOW_SCALAR_AUTOMATON.traverse(String::from("1"), i);
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn try_string1() {
        let mut rand = PRandomizer::new(100);
        for _ in 0..1000 {
            let res: String = super::FLOW_SCALAR_AUTOMATON.generate(rand.get());
            println!("Res is: {}", res);
        }
    }
}
