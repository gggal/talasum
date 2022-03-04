use super::super::helper::*;
use crate::{
    configuration::Configurable,
    state_machine::{json::whitespace::START_WS, weights::CONFIG, Automaton, AutomatonNode},
};

lazy_static! {
    static ref START_STRING: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (1, &EMPTY_STRING),
            (1, &LONG_STRING),
            (5, &NON_EMPTY_STRING)
        ]);
    static ref EMPTY_STRING: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(|_, _| { String::from("\"\"") });
    static ref NON_EMPTY_STRING: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (5, &START_WS),
            (1, &SINGLE_QUOTED_STRING),
            (1, &UNQUOTED_STRING),
            (5, &ADD_VALID_CHAR),
            (5, &ADD_INVALID_CHAR),
            (1, &REMOVE_CHAR),
            (1, &REPLACE_CHAR),
        ]);
    static ref SINGLE_QUOTED_STRING: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(|_, text| text.replace("\"", "\'"));
    static ref LONG_STRING: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(|num, _| {
            let text = String::from(&CONFIG.get_common_words()[(num % 1000) as usize]);
            format!("\"{}\"", text.repeat((num % 1024_u64) as usize))
        });
    static ref UNQUOTED_STRING: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(|_, text| text.replace("\"", ""));
    static ref ADD_VALID_UNESCAPED_CHAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|seed, text| { insert_random_char_in_string(seed, &text) })
        .set_cycle(2);
    static ref ADD_UNESCAPED_QUOTATION_MARK: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(3)
        .set_func(|seed, text| insert_string_in_string(seed, &text, "\""));
    static ref ADD_UNESCAPED_REVERSE_SOLIDUS: AutomatonNode<String> =
        AutomatonNode::<String>::new()
            .set_cycle(3)
            .set_func(|seed, text| insert_string_in_string(seed, &text, "\\"));
    static ref ADD_ESCAPED_CHARACTER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(2)
        .set_func(|seed, text| {
            let escaped = ["\\\"", "\\\\", "\\/", "\\b", "\\f", "\\\n", "\\\r", "\\\t"];
            insert_string_in_string(seed, &text, escaped[(seed % 8) as usize])
        });
    static ref ADD_ENCODED: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![
            (1, &ADD_ENCODED_RANDOM_CASE),
            (1, &ADD_ENCODED_UPPER_CASE),
            (1, &ADD_ENCODED_LOWER_CASE)
        ])
        .set_func(|seed, text| { insert_random_encoded_char_in_string(seed, &text) });
    static ref ADD_ENCODED_RANDOM_CASE: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(3)
        .set_func(|seed, text| { random_capitalization(seed, text) });
    static ref ADD_ENCODED_UPPER_CASE: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(3)
        .set_func(|seed, text| { to_upper_case(seed, text) });
    static ref ADD_ENCODED_LOWER_CASE: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_cycle(3);
    static ref REMOVE_CHAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|seed, text| {
            if text.is_empty() {
                text
            } else {
                let to_drop = (seed % text.len() as u64) as usize;
                text.char_indices()
                    .filter(|&(i, _)| i != to_drop)
                    .map(|c| c.1)
                    .collect::<String>()
            }
        })
        .set_cycle(1);
    static ref REPLACE_CHAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|seed, text| {
            if text.is_empty() {
                text
            } else {
                let to_update = (seed % text.len() as u64) as usize;
                text.char_indices()
                    .map(|(i, c)| {
                        if i == to_update {
                            format!("\\u{:04x}", c as u32)
                        } else {
                            String::from(c)
                        }
                    })
                    .collect::<String>()
            }
        })
        .set_cycle(1);
    static ref ADD_UNPAIRED_SURROGATE: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(2)
        .set_func(|seed, text| insert_random_surrogate_in_string(seed, &text));
    static ref ADD_SURROGATE_PAIR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(2)
        .set_func(|seed, text| insert_random_surrogate_pair_in_string(seed, &text));
    static ref ADD_INVALID_CHAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![(1, &ADD_UNESCAPED_CHAR), (1, &ADD_UNPAIRED_SURROGATE)]);
    static ref ADD_UNESCAPED_CHAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![
            (1, &ADD_UNESCAPED_QUOTATION_MARK),
            (1, &ADD_UNESCAPED_REVERSE_SOLIDUS),
            (1, &ADD_UNESCAPED_CONTROL_CHAR),
        ]);
    static ref ADD_UNESCAPED_CONTROL_CHAR: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|seed, text| insert_random_unescaped_control_char(seed, &text))
        .set_cycle(3);
    static ref ADD_VALID_CHAR: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (1, &ADD_VALID_UNESCAPED_CHAR),
            (1, &ADD_ESCAPED_CHARACTER),
            (1, &ADD_SURROGATE_PAIR),
            (1, &ADD_ENCODED)
        ]);
    pub static ref STRING_AUTOMATON: Automaton<String> = Automaton::<String> {
        initial_node: &START_STRING,
        generator: |seed| {
            format!(
                "\"{}\"",
                String::from(&CONFIG.get_common_words()[(seed % 1000) as usize])
            )
        },
    };
}

#[cfg(test)]
mod tests {
    use crate::randomness::{PRandomizer, Randomizer};

    use super::STRING_AUTOMATON;
    use itertools::Itertools;

    lazy_static! {
        // sorted list of a 1000 fuzzed number values
        static ref TEST_FUZZ_VALUES: Vec<String> = (1..1000)
            .map(|i| STRING_AUTOMATON.generate(i))
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
            let res: String = super::STRING_AUTOMATON.traverse(String::from("1"), i);
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn try_string1() {
        let mut rand = PRandomizer::new(100);
        for _ in 0..1000 {
            let res: String = super::STRING_AUTOMATON.generate(rand.get());
            println!("Res is: {}", res);
        }
    }
}
