//! talasum is a library for protocol fuzzing. Currently the supported protocols
//! are JSON and YAML.
//! It supports both generation-based and mutation-based fuzzing.
//! For generation-based fuzzing one needs to specify a protocol <-> type
//! pair, e.g. the number type of the JSON protocol.
//! For protocol-based fuzzing one needs to provide a valid JSON/YAML document.
//!
//! talasum's fuzzing algorithm understands the underlying protocols and can
//! effectively process nested input.
//!
//! talasum is designed to be used primarily for security testing, hence
//! it performs techniques like shellcode injection, sql injection and more,
//! as part of the fuzzing process. This behavior is configurable and can
//! be disabled by TODO.
//!

mod configuration;
mod generator;
pub mod json;
mod mutator;
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
