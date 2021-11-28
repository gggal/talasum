// #![feature(async_stream)]

//! Magi is a fuzzing library for generational and modification fuzzing.
//! It provides support for generic types and structures, as well as the following protocols/DSL:
//! http, json, more

use state_machine::json::boolean::BOOL_AUTOMATON;
use state_machine::json::number::NUMBER_AUTOMATON;

mod configuration;
pub mod generator;
pub mod mutator;
mod randomization;
mod state_machine;
mod tokenizer;

extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate config;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use crate::generator::Generator;
use crate::mutator::Mutator;

pub mod json {
    use crate::tokenizer::json_lexer::{JsonLexer, Rule};

    pub mod number {
        pub fn generator(seed: u32) -> super::super::Generator<String> {
            super::super::Generator::<String>::new(&super::super::NUMBER_AUTOMATON, seed)
        }
    }

    pub mod boolean {
        pub fn generator(seed: u32) -> super::super::Generator<String> {
            super::super::Generator::<String>::new(&super::super::BOOL_AUTOMATON, seed)
        }
    }

    pub fn mutator(input: &'static str, seed: u32) -> Option<super::Mutator> {
        super::Mutator::new::<JsonLexer, Rule>(seed, input, Rule::value)
    }
}
