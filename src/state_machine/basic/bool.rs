// use crate::state_machine::{Automaton, AutomatonEdge, AutomatonState};

// #[derive(Default, Debug)]
// struct BoolAutomaton {
//     val: String,
// }

// impl BoolAutomaton {
//     fn get_start_state() -> Box<dyn AutomatonState<String>> {
//         return Box::new(StartState {});
//     }
// }

// impl Automaton<String> for BoolAutomaton {
//     fn init_value(&self) -> String {
//         self.val.clone()
//     }

//     fn init_state(&self) -> Box<dyn AutomatonState<String>> {
//         Self::get_start_state()
//     }
// }

// // #[derive(Default)]
// struct StartState;
// impl AutomatonState<String> for StartState {
//     fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
//         match seed % 100 {
//             0..=10 => Some((Box::new(SecState), |_: String| {String::from("False")})),
//             11..=20 => Some((Box::new(SecState), |_: String| {String::from("True")})),
//             21..=30 => Some((Box::new(SecState), |_: String| {String::from("true")})),
//             31..=40 => Some((Box::new(SecState), |_: String| {String::from("false")})),
//             41..=50 => Some((Box::new(SecState), |_: String| {String::from("TRUE")})),
//             51..=60 => Some((Box::new(SecState), |_: String| {String::from("FALSE")})),
//             61..=70 => Some((Box::new(SecState), |_: String| {String::from("1")})),
//             71..=100 => Some((Box::new(SecState), |_: String| {String::from("0")})),
//             // 81..=100 => Some((Box::new(FinalState), |x: String| {String::from("FALSE")})),
//             _ => panic!("Invalid seed"),
//         }
//     }
// }

// struct SecState;
// impl AutomatonState<String> for SecState {
//     fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<String>> {
//         match seed % 100 {
//             1..=20 => Some((Box::new(FinalState), |x: String| {format!("{}{}", x, "asd")})),
//             21..=100 => None,
//             _ => panic!("Invalid seed"),
//         }
//     }
// }

// struct FinalState;
// impl AutomatonState<String> for FinalState {
//     fn decide_next(&self, _: u32) -> Option<AutomatonEdge<String>> {
//         None
//     }
// }

// // #[cfg(test)]
// // mod tests {
// //     use crate::state_machine::AutomatonState;

// //     #[test]
// //     #[should_panic(expected = "Invalid seed")]
// //     fn panic_when_seed_is_invalid() {
// //         super::StartState.decide_next(123);
// //     }
// // }
