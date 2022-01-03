use num_bigint::BigUint;

use super::super::helper::*;
use crate::state_machine::{json::whitespace::START_WS, Automaton, AutomatonNode};

lazy_static! {
    static ref START_NUMBER: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (8, &WHOLE_NUMBER),
            (1, &super::null::START_NULL),
            (1, &HEX_NUMBER),
            (1, &OCTAL_NUMBER),
            (1, &NA_NUMBER),
            (1, &INFINITE_NUMBER),
        ]);
    static ref REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![
            (1, &INCREASED_PRECISION_REAL_NUMBER),
            (1, &SCI_NOTATION_REAL_NUMBER),
            (1, &DECIMAL_COMMA_REAL_NUMBER),
            (2, &START_WS)
        ])
        .set_func(|num| num
            .parse::<f64>()
            .expect("Invalid automaton definition")
            .to_string());
    static ref INCREASED_PRECISION_REAL_NUMBER: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_cycle(1).set_func(|num| {
            num.parse::<f64>()
                .expect("Invalid automaton definition")
                .sqrt()
                .to_string()
        });
    static ref DECIMAL_COMMA_REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edge(&START_WS)
        .set_func(|num| str::replace(&num, ".", ","));
    static ref WHOLE_NUMBER: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![
            (2, &POSITIVE_NUMBER),
            (2, &NEGATIVE_NUMBER),
            (2, &POWER_NATURAL_NUMBER),
            (2, &SUM_NATURAL_NUMBER),
            (1, &OVERFLOWED_WHOLE_NUMBER)
        ]);
    static ref OVERFLOWED_WHOLE_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edges(vec![
            (2, &FINAL),
            (1, &OVERFLOWED_REAL_NUMBER),
            (1, &POWER_OVERFLOWED_NATURAL_NUMBER),
            (1, &SUM_OVERFLOWED_NATURAL_NUMBER)
        ]);
    static ref POWER_OVERFLOWED_NATURAL_NUMBER: AutomatonNode<String> =
        AutomatonNode::<String>::new()
            .set_cycle(1)
            .set_func(|input| {
                input
                    .parse::<BigUint>()
                    .expect("Invalid automaton definition")
                    .pow(2)
                    .to_string()
            });
    static ref SUM_OVERFLOWED_NATURAL_NUMBER: AutomatonNode<String> =
        AutomatonNode::<String>::new()
            .set_cycle(1)
            .set_func(|input| { format!("{}1", input) });
    static ref OVERFLOWED_REAL_NUMBER: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(|input| { format!("0.{}", input) });
    static ref POWER_NATURAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(1)
        .set_func(|input| {
            match input.parse::<u128>() {
                Ok(num) if num < u32::MAX as u128 => num.pow(2).to_string(),
                Ok(_) => u64::MAX.to_string(),
                Err(_) => panic!("Invalid automaton definition"),
            }
        });
    static ref SUM_NATURAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_cycle(1)
        .set_func(|input| {
            match input.parse::<u128>() {
                Ok(num) if num < u64::MAX as u128 / 2 => (num * 2).to_string(),
                Ok(_) => u64::MAX.to_string(),
                Err(_) => panic!("Invalid automaton definition"),
            }
        });
    static ref HEX_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|input| format!("{:#01x}", input.parse::<u64>().unwrap()));
    static ref OCTAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|input| format!("0{:o}", input.parse::<u64>().unwrap()));
    static ref NA_NUMBER: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(|_| String::from("NaN"));
    static ref INFINITE_NUMBER: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_func(|_| String::from("∞"));
    static ref SCI_NOTATION_REAL_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_edge(&START_WS)
        .set_func(|num| format!("{:+e}", num.parse::<f64>().unwrap()));
    static ref NEGATIVE_NUMBER: AutomatonNode<String> = AutomatonNode::<String>::new()
        .set_func(|num| format!("-{}", num))
        .set_edges(vec![(1, &FINAL), (1, &REAL_NUMBER)]);
    static ref POSITIVE_NUMBER: AutomatonNode<String> =
        AutomatonNode::<String>::new().set_edges(vec![(1, &FINAL), (1, &REAL_NUMBER)]);
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
        for i in 1..50 {
            let res: String = super::NUMBER_AUTOMATON.generate(i);
            println!("Res is: {}", res);
        }
    }
}
