use super::super::helper::*;
use super::super::weights::choose;
use crate::state_machine::{Automaton, AutomatonNode};

lazy_static! {
    static ref START_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![
            (4, Some(&REAL_NUMBER)),
            (4, Some(&NATURAL_NUMBER)),
            (1, Some(&super::null::START_NULL)),
            (1, Some(&HEX_NUMBER)),
            (1, Some(&OCTAL_NUMBER)),
            (1, Some(&NA_NUMBER)),
            (1, Some(&INFINITE_NUMBER))
        ]),
        transformation: IDENTITY,
    };
    static ref REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![
            (10, Some(&SCI_NOTATION_REAL_NUMBER)),
            (35, Some(&SIGNED_NUMBER)),
            (15, Some(&DECIMAL_COMMA_REAL_NUMBER)),
            (40, None)
        ]),
        transformation: |num| {
            let num1 = num.parse::<u64>().unwrap();
            let delim = num1 % 10 + 1;
            (num1 / (100 * delim)).to_string()
        },
    };
    static ref DECIMAL_COMMA_REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![(1, Some(&SIGNED_NUMBER)), (1, None)]),
        transformation: |num| str::replace(&num, ".", ","),
    };
    static ref NATURAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![(1, Some(&SIGNED_NUMBER)), (1, None)]),
        transformation: IDENTITY,
    };
    static ref HEX_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![]),
        transformation: |input| format!("{:#01x}", input.parse::<u64>().unwrap()),
    };
    static ref OCTAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![]),
        transformation: |input| format!("0{:o}", input.parse::<u64>().unwrap()),
    };
    static ref NA_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![]),
        transformation: |_| String::from("NaN"),
    };
    static ref INFINITE_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![]),
        transformation: |_| String::from("∞"),
    };
    static ref SCI_NOTATION_REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![(1, Some(&SIGNED_NUMBER)), (1, None)]),
        transformation: |num| format!("{:+e}", num.parse::<u64>().unwrap()),
    };
    static ref SIGNED_NUMBER: AutomatonNode<String> = AutomatonNode::<String> {
        transition: choose(vec![]),
        transformation: |num| format!("-{}", num)
    };
    pub static ref NUMBER_AUTOMATON: Automaton<String> = Automaton::<String> {
        initial_node: &START_NUMBER,
        generator: |seed| random_digit_string(seed),
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
