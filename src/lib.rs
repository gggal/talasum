//! Magi is a fuzzing library for generational and modification fuzzing.
//! It provides support for generic types and structures, as well as the following protocols/DSL:
//! http, json, more

// use syn::{Result};
// use syn::Expr;
// use syn::TypePtr;
pub mod state_machine;
// use crate::state_machine;

// pub fn a() -> Result<()>{
//     let t: TypePtr = syn::parse_str("std::collections::HashMap<String, Value>")?;

//     // println!("{:?}", t );
//     Ok(())
// }

// fn run() -> Result<()> {
//     let code = "assert_eq!(u8::max_value(), 255)";
//     let expr = syn::parse_str("assert_eq!(u8::max_value(), 255)")?;
//     println!("{:#?}", expr);
//     Ok(())
// }

// #[cfg(test)]
// mod tests {
//     use syn::{Result, Type};
//     use syn::TypePtr;

//     pub fn a() -> Result<()>{
//         let t: TypePtr = syn::parse_str("std::collections::HashMap<String, Value>")?;
//         let asd = t.elem;
//         // println!("{:?}", t );
//         Ok(())
//     }

//     fn print_type_of<T>(_: &T) -> String {
//         println!("{}", std::any::type_name::<T>());
//         format!("{}", std::any::type_name::<T>())
//     }

//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }

//     fn enum_to_txt(ty: &syn::Type) -> String {
//         match &ty {
//             syn::Type::Array(_) => "array".to_owned(),
//             syn::Type::Slice(_) => "slice".to_owned(),
//             syn::Type::Ptr(_) => "ptr".to_owned(),
//             syn::Type::Reference(_) => "Reference".to_owned(),
//             syn::Type::BareFn(_) => "BareFn".to_owned(),
//             syn::Type::Never(_) => "Never".to_owned(),
//             syn::Type::Tuple(_) => "Tuple".to_owned(),
//             syn::Type::Path(_) => "Path".to_owned(),
//             syn::Type::TraitObject(_) => "TraitObject".to_owned(),
//             syn::Type::ImplTrait(_) => "ImplTrait".to_owned(),
//             syn::Type::Paren(_) => "Paren".to_owned(),
//             syn::Type::Group(_) => "Group".to_owned(),
//             syn::Type::Infer(_) => "Infer".to_owned(),
//             syn::Type::Macro(_) => "Macro".to_owned(),
//             syn::Type::Verbatim(_) => "Verbatim".to_owned(),
//             _ => "other..".to_owned()
//         }
//     }

//     #[test]
//     fn syn_test() {
//         let a = 23;
//         //std::collections::HashMap<String, String>
//         match syn::parse_str::<syn::Type>("fn(asd) -> asd") {
//             Err(t) =>  {
//                 assert_eq!(print_type_of(&t), "")
//             }
//             Ok(t) => {
//                 assert_eq!(print_type_of(&t), "syn::ty::Type");
//                 assert_eq!(enum_to_txt(&t), "left is the type")
//             }
//         }
//     }
// }
