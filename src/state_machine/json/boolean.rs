use super::randomization::*;
use crate::state_machine::{Automaton, AutomatonNode};

#[allow(dead_code)]
static START_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed: u32| match seed % 100 {
        0..=80 => None,
        _ => Some(&REVERSE_BOOLEAN),
    },
    transformation: super::IDENTITY,
};

static REVERSE_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |seed| match seed % 100 {
        0..=50 => Some(&NUMERICAL_BOOLEAN),
        _ => Some(&CASED_BOOLEAN),
    },
    transformation: |_, input| {
        if input == "true" {
            String::from("false")
        } else {
            String::from("true")
        }
    },
};

static NUMERICAL_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| Some(&QUOTED_BOOLEAN),
    transformation: |seed, _| (seed % 2).to_string(),
};

static QUOTED_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| Some(&CASED_BOOLEAN),
    transformation: |seed, text| match seed % 100 {
        0..=20 => format!("\"{}\"", text),
        _ => text,
    },
};

static CASED_BOOLEAN: AutomatonNode<String> = AutomatonNode::<String> {
    transition: |_| None,
    transformation: |seed, text| match seed % 100 {
        0..=10 => to_upper_case(seed, text),
        11..=20 => to_capitalized(seed, text),
        21..=30 => to_random_case(seed, text),
        _ => text,
    },
};

#[allow(dead_code)]
pub static BOOL_AUTOMATON: Automaton<String> = Automaton::<String> {
    initial_node: &START_BOOLEAN,
    generator: |seed| {
        if seed % 2 == 0 {
            String::from("true")
        } else {
            String::from("false")
        }
    },
};

#[cfg(test)]
mod tests {
    use super::BOOL_AUTOMATON;

    #[test]
    fn try_bool() {
        for _i in 1..20 {
            let res: String = BOOL_AUTOMATON.traverse(String::from("false"));
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn try_bool1() {
        for _i in 1..20 {
            let res: String = BOOL_AUTOMATON.generate();
            println!("Res is: {}", res);
        }
    }

    #[test]
    fn capitalized_works() {
        assert_eq!(
            super::to_capitalized(123, String::from("word")),
            String::from("Word")
        );
    }
}
