// use crate::state_machine::{Automaton, AutomatonEdge, AutomatonState};

// #[derive(Default, Debug)]
// pub struct StringAutomaton {
//     val: String,
// }

// impl StringAutomaton {
//     pub fn new() -> Self {
//         Self {
//             val: super::randomization::random_digit_string(123),
//         }
//     }

//     pub fn new_from_val(val: String) -> Self {
//         Self { val: val }
//     }

//     fn get_start_state() -> Box<dyn AutomatonState<String>> {
//         return Box::new(StartString);
//     }
// }

// impl Automaton<String> for StringAutomaton {
//     fn init_value(&self) -> String {
//         self.val.clone()
//     }

//     fn init_state(&self) -> Box<dyn AutomatonState<String>> {
//         Self::get_start_state()
//     }
// }

// // insert unescaped at random position
// // empty string
// // string without quotes
// // string with single quotes
// // too long string
// // different encoding
// pub struct StartString;
// impl AutomatonState<String> for StartString {
//     fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
//         match seed % 100 {
//             0..=40 => Some((Box::new(LiteralNull), |_, _| String::from("null"))),
//             41..=50 => Some((Box::new(LiteralNull), |_, _| String::from("Null"))),
//             51..=60 => Some((Box::new(LiteralNull), |_, _| String::from("nil"))),
//             61..=70 => Some((Box::new(LiteralNull), |_, _| String::from("Nil"))),
//             71..=80 => Some((Box::new(LiteralNull), |_, _| String::from("none"))),
//             81..=90 => Some((Box::new(LiteralNull), |_, _| String::from("None"))),
//             91..=100 => Some((Box::new(LiteralNull), |_, _| String::from("0"))),
//             _ => panic!("Invalid seed"),
//         }
//     }
// }

// struct LiteralNull;
// impl AutomatonState<String> for LiteralNull {
//     fn decide_next(&self, _seed: u32) -> Option<AutomatonEdge<String>> {
//         None
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::state_machine::Automaton;
//     use crate::state_machine::AutomatonState;

//     // #[test]
//     // #[should_panic(expected = "Invalid seed")]
//     // fn panic_when_seed_is_invalid() {
//     //     super::StartNull.decide_next(123);
//     // }

//     // #[test]
//     // fn try_null() {
//     //     let mut my_machine: super::NullAutomaton = super::NullAutomaton::default();
//     //     for _i in 1..20 {
//     //         let res = my_machine.traverse();
//     //         println!("Res is: {}", res);
//     //     }
//     //     super::StartNull.decide_next(123);
//     // }
// }
