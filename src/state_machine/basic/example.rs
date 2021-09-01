// use crate::state_machine::{Automaton, AutomatonEdge, AutomatonState};

// #[derive(Default, Debug)]
// struct ExampleAutomaton {
//     val: i32,
// }

// impl ExampleAutomaton {
//     fn get_start_state() -> Box<dyn AutomatonState<i32>> {
//         return Box::new(StartState {});
//     }
// }

// impl Automaton<i32> for ExampleAutomaton {
//     fn init_value(&self) -> i32 {
//         self.val
//     }

//     fn init_state(&self) -> Box<dyn AutomatonState<i32>> {
//         Self::get_start_state()
//     }
// }

// struct StartState;
// impl AutomatonState<i32> for StartState {
//     fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<i32>> {
//         match seed % 100 {
//             0..=20 => Some((Box::new(self::EndState), operation)),
//             21..=100 => Some((Box::new(self::EndState), operation)),
//             _ => panic!("at the disco"),
//         }
//     }
// }

// fn operation(input: i32) -> i32 {
//     input + 1
// }

// struct EndState;
// impl AutomatonState<i32> for EndState {
//     fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<i32>> {
//         match seed % 100 {
//             1..=100 => None,
//             _ => panic!("at the disco"),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     // use syn::TypePtr;
//     // use syn::{Result, Type};

//     // pub fn a() -> Result<()> {
//     //     let t: TypePtr = syn::parse_str("std::collections::HashMap<String, Value>")?;
//     //     let asd = t.elem;
//     //     // println!("{:?}", t );
//     //     Ok(())
//     // }

//     // fn print_type_of<T>(_: &T) -> String {
//     //     println!("{}", std::any::type_name::<T>());
//     //     format!("{}", std::any::type_name::<T>())
//     // }

//     // #[test]
//     // fn it_works() {
//     //     assert_eq!(2 + 2, 4);
//     // }

//     // fn enum_to_txt(ty: &syn::Type) -> String {
//     //     match &ty {
//     //         syn::Type::Array(_) => "array".to_owned(),
//     //         syn::Type::Slice(_) => "slice".to_owned(),
//     //         syn::Type::Ptr(_) => "ptr".to_owned(),
//     //         syn::Type::Reference(_) => "Reference".to_owned(),
//     //         syn::Type::BareFn(_) => "BareFn".to_owned(),
//     //         syn::Type::Never(_) => "Never".to_owned(),
//     //         syn::Type::Tuple(_) => "Tuple".to_owned(),
//     //         syn::Type::Path(_) => "Path".to_owned(),
//     //         syn::Type::TraitObject(_) => "TraitObject".to_owned(),
//     //         syn::Type::ImplTrait(_) => "ImplTrait".to_owned(),
//     //         syn::Type::Paren(_) => "Paren".to_owned(),
//     //         syn::Type::Group(_) => "Group".to_owned(),
//     //         syn::Type::Infer(_) => "Infer".to_owned(),
//     //         syn::Type::Macro(_) => "Macro".to_owned(),
//     //         syn::Type::Verbatim(_) => "Verbatim".to_owned(),
//     //         _ => "other..".to_owned(),
//     //     }
//     // }
// }
