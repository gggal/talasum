//! Magi is a library for protocol fuzzing. Currently the supported protocols
//! are JSON and YAML.
//! It supports both generation-based and mutation-based fuzzing.
//! For generation-based fuzzing one needs to specify a protocol <-> type
//! pair, e.g. the number type of the JSON protocol.
//! For protocol-based fuzzing one needs to provide a valid JSON/YAML document.
//!
//! Magi's fuzzing algorithm understands the underlying protocols and can 
//! effectively process nested input.
//!
//! Magi is designed to be used primarily for security testing, hence
//! it performs techniques like shellcode injection, sql injection and more,
//! as part of the fuzzing process. This behavior is configurable and can
//! be disabled by TODO.
//!

use state_machine::json::boolean::BOOL_AUTOMATON;
use state_machine::json::number::NUMBER_AUTOMATON;

mod configuration;
pub mod generator;
pub mod mutator;
mod randomness;
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

/// This module contains all JSON fuzzers
pub mod json {
    use crate::randomness::PRandomizer;
    use crate::tokenizer::json_lexer::{JsonLexer, Rule};

    /// Returns a JSON Number generator
    ///
    /// # Examples
    /// ```rust
    /// use magi::json;
    ///
    /// for fuzzed in json::number(1234).take(10) {
    ///     println!("New number value: {}", fuzzed);
    /// }
    /// ```
    pub fn number(seed: u64) -> super::Generator<String> {
        super::Generator::<String>::new(&super::NUMBER_AUTOMATON, Box::new(PRandomizer::new(seed)))
    }

    /// Returns a JSON Boolean generator
    ///
    /// # Examples
    /// ```rust
    /// use magi::json;
    ///
    /// for fuzzed in json::boolean(1234).take(10) {
    ///     println!("New boolean value: {}", fuzzed);
    /// }
    /// ```
    pub fn boolean(seed: u64) -> super::Generator<String> {
        super::Generator::<String>::new(&super::BOOL_AUTOMATON, Box::new(PRandomizer::new(seed)))
    }

    /// Returns a JSON Mutator
    ///
    /// # Examples
    /// ```rust
    /// use magi::json;
    ///
    /// match json::mutate("{\"a\": 123, \"b\": [null, true, \"c\"]}", 1234) {
    ///     Some(mutator) => {
    ///         for fuzzed in mutator.take(10) {
    ///             println!("New value: {}", fuzzed);
    ///         }
    ///     },
    ///     None => panic!("Your input was not a valid JSON document")
    /// }
    ///
    /// ```
    pub fn mutate(input: &'static str, seed: u64) -> Option<super::Mutator> {
        super::Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(seed)), input, Rule::value)
    }
}
