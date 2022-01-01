use super::super::helper::*;
use crate::state_machine::{Automaton, AutomatonNode};

lazy_static! {
    static ref START_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new().set_edges(
        vec![
            (4, &REAL_NUMBER),
            (4, &NATURAL_NUMBER),
            (1, &super::null::START_NULL),
            (1, &HEX_NUMBER),
            (1, &OCTAL_NUMBER),
            (1, &NA_NUMBER),
            (1, &INFINITE_NUMBER),
        ]);
    static ref REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new().set_edges(
        vec![
            (10, &SCI_NOTATION_REAL_NUMBER),
            (35, &SIGNED_NUMBER),
            (15, &DECIMAL_COMMA_REAL_NUMBER),
            (40, &FINAL)
        ]).set_func(|num| {
            let num1 = num.parse::<u64>().unwrap();
            let delim = num1 % 10 + 1;
            (num1 / (100 * delim)).to_string()
        });
    static ref DECIMAL_COMMA_REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new().set_edges(
        vec![
            (1, &SIGNED_NUMBER),
            (1, &FINAL)
        ]).set_func(|num| str::replace(&num, ".", ","));
    static ref NATURAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new().set_edges(vec![
        (1, &SIGNED_NUMBER),
        (1, &FINAL)
    ]);
    static ref HEX_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|input| format!("{:#01x}", input.parse::<u64>().unwrap()));
    static ref OCTAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|input| format!("0{:o}", input.parse::<u64>().unwrap()));
    static ref NA_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_| String::from("NaN"));
    static ref INFINITE_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|_| String::from("∞"));
    static ref SCI_NOTATION_REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![
            (1, &SIGNED_NUMBER),
            (1, &FINAL)
        ]).set_func( |num| format!("{:+e}", num.parse::<u64>().unwrap()));
    static ref SIGNED_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|num| format!("-{}", num));
    pub static ref NUMBER_AUTOMATON: Automaton<String> = Automaton::<String> {
        initial_node: &START_NUMBER,
        generator: random_digit_string,
    };
}

#[cfg(test)]
mod tests {
    use super::NUMBER_AUTOMATON;
    use itertools::Itertools;

    // leading zeros not allowed by rfc
    // double precision
    // u64/i64 overflow

    lazy_static! {
        // sorted list of a 1000 fuzzed number values
        static ref TEST_FUZZ_VALUES: Vec<String> = (1..1000)
            .map(|i| NUMBER_AUTOMATON.generate(i))
            .sorted()
            .collect();
    }

    #[test]
    fn number_automaton_is_seedable() {
        assert_ne!(TEST_FUZZ_VALUES.last(), TEST_FUZZ_VALUES.first());
    }

    #[test]
    fn result_is_diverse_enough() {
        let unique_values = TEST_FUZZ_VALUES.iter().unique().count();
        assert!(unique_values > 15);
    }

    #[test]
    fn try_number() {
        for i in 1..20 {
            let res: String = super::NUMBER_AUTOMATON.traverse(String::from("1"), i);
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn try_number1() {
        for i in 1..20 {
            let res: String = super::NUMBER_AUTOMATON.generate(i);
            println!("Res is: {}", res);
        }
    }
}
