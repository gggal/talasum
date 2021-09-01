use crate::state_machine::{Automaton, AutomatonEdge, AutomatonState};

#[derive(Default, Debug)]
pub struct BooleanAutomaton {
    val: String,
}

impl BooleanAutomaton {
    fn get_start_state() -> Box<dyn AutomatonState<String>> {
        return Box::new(StartBoolean);
    }
}

impl Automaton<String> for BooleanAutomaton {
    fn init_value(&self) -> String {
        self.val.clone()
    }

    fn init_state(&self) -> Box<dyn AutomatonState<String>> {
        Self::get_start_state()
    }
}

struct StartBoolean;
impl AutomatonState<String> for StartBoolean {
    fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
        match seed % 100 {
            0..=40 => Some((Box::new(LiteralBoolean), |_, _| String::from("false"))),
            41..=80 => Some((Box::new(LiteralBoolean), |_, _| String::from("true"))),
            81..=85 => Some((Box::new(NumericalBoolean), |_, _| String::from("1"))),
            86..=90 => Some((Box::new(NumericalBoolean), |_, _| String::from("0"))),
            91..=100 => None, //Some((Box::new(NumericalBoolean), super::IDENTITY)),
            _ => panic!("Invalid seed"),
        }
    }
}

struct LiteralBoolean;
impl AutomatonState<String> for LiteralBoolean {
    fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
        match seed % 100 {
            0..=20 => Some((Box::new(QuotedLiteralBoolean), |_, x: String| {
                format!("\"{}\"", x)
            })),
            21..=100 => Some((Box::new(QuotedLiteralBoolean), super::IDENTITY)),
            _ => panic!("Invalid seed"),
        }
    }
}

struct QuotedLiteralBoolean;
impl AutomatonState<String> for QuotedLiteralBoolean {
    fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
        match seed % 100 {
            0..=30 => Some((Box::new(CasedBoolean), to_upper_case)),
            31..=60 => Some((Box::new(CasedBoolean), to_capitalized)),
            61..=90 => Some((Box::new(CasedBoolean), to_random_case)),
            91..=100 => Some((Box::new(super::null::StartNull), super::IDENTITY)),
            _ => panic!("Invalid seed"),
        }
    }
}

struct NumericalBoolean;
impl AutomatonState<String> for NumericalBoolean {
    fn decide_next(&self, _seed: u32) -> Option<AutomatonEdge<String>> {
        None
    }
}

struct CasedBoolean;
impl AutomatonState<String> for CasedBoolean {
    fn decide_next(&self, _: u32) -> Option<AutomatonEdge<String>> {
        None
    }
}

fn to_upper_case(_seed: u32, s: String) -> String {
    s.to_ascii_uppercase()
}

fn to_capitalized(_seed: u32, s: String) -> String {
    let mut to_return = s.clone();
    to_return[0..1].make_ascii_uppercase();
    // print!("Debug {}", to_return);
    to_return
}

fn to_random_case(seed: u32, s: String) -> String {
    super::randomization::random_capitalization(seed, s)
}

#[cfg(test)]
mod tests {
    use crate::state_machine::Automaton;
    use crate::state_machine::AutomatonState;

    // #[test]
    // #[should_panic(expected = "Invalid seed")]
    // fn panic_when_seed_is_invalid() {
    //     super::StartBoolean.decide_next(123);
    // }

    #[test]
    fn try_bool() {
        let mut my_machine: super::BooleanAutomaton = super::BooleanAutomaton::default();
        for _i in 1..20 {
            let res = my_machine.traverse();
            println!("Res is: {}", res);
        }
        super::StartBoolean.decide_next(123);
    }

    #[test]
    fn capitalized_works() {
        assert_eq!(
            super::to_capitalized(123, String::from("word")),
            String::from("Word")
        );
    }

    // const SAMPLE : [String] = for _i in 1..20 {
    //     my_machine.traverse(i)
    // };

    // #[test]
    // fn generated_capital_case() {

    // }
}
