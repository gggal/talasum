// #![feature(async_stream)]

//! Magi is a fuzzing library for generational and modification fuzzing.
//! It provides support for generic types and structures, as well as the following protocols/DSL:
//! http, json, more

// use syn::{Result};
// use syn::Expr;
// use syn::TypePtr;
use rand::{Rng, SeedableRng};
use rand_pcg::{Lcg64Xsh32, Pcg32};

pub mod png;
pub mod state_machine;
pub mod tokenizer;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

struct IdentParser;
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

fn mute_not_used_warnings() {
    let res = tokenizer::tokenize_input::<
        tokenizer::json_lexer::JsonLexer,
        tokenizer::json_lexer::Rule,
    >(&String::from("1"), tokenizer::json_lexer::Rule::value);
    for el in res {
        println!("from {} ", el.from);
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use rand_pcg::Pcg32;
    use syn::TypePtr;
    use syn::{Result, Type};

    pub fn a() -> Result<()> {
        let t: TypePtr = syn::parse_str("std::collections::HashMap<String, Value>")?;
        let asd = t.elem;
        // println!("{:?}", t );
        Ok(())
    }

    fn print_type_of<T>(_: &T) -> String {
        println!("{}", std::any::type_name::<T>());
        format!("{}", std::any::type_name::<T>())
    }

    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }

    // fn enum_to_txt(ty: &syn::Type) -> String {
    //     match &ty {
    //         syn::Type::Array(_) => "array".to_owned(),
    //         syn::Type::Slice(_) => "slice".to_owned(),
    //         syn::Type::Ptr(_) => "ptr".to_owned(),
    //         syn::Type::Reference(_) => "Reference".to_owned(),
    //         syn::Type::BareFn(_) => "BareFn".to_owned(),
    //         syn::Type::Never(_) => "Never".to_owned(),
    //         syn::Type::Tuple(_) => "Tuple".to_owned(),
    //         syn::Type::Path(_) => "Path".to_owned(),
    //         syn::Type::TraitObject(_) => "TraitObject".to_owned(),
    //         syn::Type::ImplTrait(_) => "ImplTrait".to_owned(),
    //         syn::Type::Paren(_) => "Paren".to_owned(),
    //         syn::Type::Group(_) => "Group".to_owned(),
    //         syn::Type::Infer(_) => "Infer".to_owned(),
    //         syn::Type::Macro(_) => "Macro".to_owned(),
    //         syn::Type::Verbatim(_) => "Verbatim".to_owned(),
    //         _ => "other..".to_owned()
    //     }
    // }

    // #[test]
    // fn syn_test() {
    //     let a = 23;
    //     //std::collections::HashMap<String, String>
    //     match syn::parse_str::<syn::Type>("fn(asd) -> asd") {
    //         Err(t) =>  {
    //             assert_eq!(print_type_of(&t), "")
    //         }
    //         Ok(t) => {
    //             assert_eq!(print_type_of(&t), "syn::ty::Type");
    //             assert_eq!(enum_to_txt(&t), "left is the type")
    //         }
    //     }
    // }

    #[test]
    pub fn main() {
        use rand_core::SeedableRng;
        use rand_pcg::Pcg32;
        let mut rng = Pcg32::seed_from_u64(42);
        for idx in 0..10 {
            let x: u32 = rng.gen();
            println!("Num {}: {}", idx, x);
        }
    }
}

pub fn rand_num(seed: u64) -> Pcg32 {
    Pcg32::seed_from_u64(seed)
}
