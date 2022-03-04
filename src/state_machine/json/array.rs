use super::super::helper::*;
use super::boolean::BOOL_AUTOMATON;
use super::null::NULL_AUTOMATON;
use super::number::NUMBER_AUTOMATON;
use super::string::STRING_AUTOMATON;
use crate::state_machine::{Automaton, AutomatonNode};

lazy_static! {
    static ref START_ARRAY: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (5, &ADD_ELEMENT_ARRAY),
            (1, &LARGE_ARRAY),
            (5, &FINAL)
        ]);
    static ref ADD_ELEMENT_ARRAY: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![
            (1, &ADD_NULL),
            (1, &ADD_BOOL),
            (1, &ADD_NUMBER),
            (1, &ADD_STRING),
            (1, &ADD_ARRAY),
            // (1, &ADD_OBJECT),
        ]);
    static ref LARGE_ARRAY: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|num, text| {
            if text.eq("[]") {
                text
            } else {
                // remove outside brackets
                let elements = text.get(1..text.len()-1).unwrap();

                // expand the list and put the brackets back
                let expanded = [elements,", "].concat().repeat((num % 128_u64) as usize);
                format!("[{}, {}]", expanded, elements)
            }
        });
    static ref ADD_BOOL: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(2)
        .set_func(|seed, text| insert_element(seed, text, &BOOL_AUTOMATON));
    static ref ADD_NULL: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(2)
        .set_func(|seed, text| insert_element(seed, text, &NULL_AUTOMATON));
    static ref ADD_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(2)
        .set_func(|seed, text| insert_element(seed, text, &NUMBER_AUTOMATON));
    static ref ADD_STRING: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(2)
        .set_func(|seed, text| insert_element(seed, text, &STRING_AUTOMATON));
    static ref ADD_ARRAY: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(2)
        .set_func(|seed, text| insert_element(seed, text, &ARRAY_AUTOMATON));
    pub static ref ARRAY_AUTOMATON: Automaton<String> = Automaton::<String> {
        initial_node: &START_ARRAY,
        generator: |_| String::from("[]")
    };
}

fn insert_element(seed: u64, text: String, automaton: &Automaton<String>) -> String {
    let to_add: String = automaton.generate(seed);
    if text.eq("[]") {
        format!("[{}]", to_add)
    } else {
        text.replacen("[", &format!("[ {},", to_add), 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::randomness::{PRandomizer, Randomizer};

    use super::ARRAY_AUTOMATON;
    use itertools::Itertools;

    lazy_static! {
        // sorted list of a 1000 fuzzed array values
        static ref TEST_FUZZ_VALUES: Vec<String> = (1..1000)
            .map(|i| ARRAY_AUTOMATON.generate(i))
            .sorted()
            .collect();
    }

    #[test]
    fn array_automaton_is_seedable() {
        assert_ne!(TEST_FUZZ_VALUES.last(), TEST_FUZZ_VALUES.first());
    }

    #[test]
    fn result_is_diverse_enough() {
        let unique_values = TEST_FUZZ_VALUES.iter().unique().count();
        assert!(unique_values > 5);
    }

    #[test]
    fn try_array() {
        for i in 1..20 {
            let res: String = ARRAY_AUTOMATON.generate(i);
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn try_array1() {
        let mut rand = PRandomizer::new(100);
        for _ in 0..1000 {
            let res: String = super::ARRAY_AUTOMATON.generate(rand.get());
            println!("Res is: {}", res);
        }
    }
}
