use crate::configuration::Config;
use crate::generator::Generator;
use crate::mutator::Mutator;
use crate::randomness::PRandomizer;
use crate::state_machine::json::array::ARRAY_AUTOMATON;
use crate::state_machine::json::boolean::BOOL_AUTOMATON;
use crate::state_machine::json::null::NULL_AUTOMATON;
use crate::state_machine::json::number::NUMBER_AUTOMATON;
use crate::state_machine::json::object::OBJECT_AUTOMATON;
use crate::state_machine::json::string::STRING_AUTOMATON;
use crate::tokenizer::json_lexer::{JsonLexer, Rule};

/// Returns a JSON Number generator
///
/// # Examples
/// ```rust
/// use talasum::json;
///
/// for fuzzed in json::number(1234).take(10) {
///     println!("New number value: {}", fuzzed);
/// }
/// ```
pub fn number(seed: u64) -> Generator<String> {
    Generator::<String>::new(&NUMBER_AUTOMATON, Box::new(PRandomizer::new(seed)))
}

/// Returns a JSON Boolean generator
///
/// # Examples
/// ```rust
/// use talasum::json;
///
/// for fuzzed in json::boolean(1234).take(10) {
///     println!("New boolean value: {}", fuzzed);
/// }
/// ```
pub fn boolean(seed: u64) -> Generator<String> {
    Generator::<String>::new(&BOOL_AUTOMATON, Box::new(PRandomizer::new(seed)))
}

/// Returns a JSON Null generator
///
/// # Examples
/// ```rust
/// use talasum::json;
///
/// for fuzzed in json::null(1234).take(10) {
///     println!("New null value: {}", fuzzed);
/// }
/// ```
pub fn null(seed: u64) -> Generator<String> {
    Generator::<String>::new(&NULL_AUTOMATON, Box::new(PRandomizer::new(seed)))
}

/// Returns a JSON String generator
///
/// # Examples
/// ```rust
/// use talasum::json;
///
/// for fuzzed in json::string(1234).take(10) {
///     println!("New string value: {}", fuzzed);
/// }
/// ```
pub fn string(seed: u64) -> Generator<String> {
    Generator::<String>::new(&STRING_AUTOMATON, Box::new(PRandomizer::new(seed)))
}

/// Returns a JSON Array generator
///
/// # Examples
/// ```rust
/// use talasum::json;
///
/// for fuzzed in json::array(1234).take(10) {
///     println!("New array value: {}", fuzzed);
/// }
/// ```
pub fn array(seed: u64) -> Generator<String> {
    Generator::<String>::new(&ARRAY_AUTOMATON, Box::new(PRandomizer::new(seed)))
}

/// Returns a JSON Object generator
///
/// # Examples
/// ```rust
/// use talasum::json;
///
/// for fuzzed in json::object(1234).take(10) {
///     println!("New object value: {}", fuzzed);
/// }
/// ```
pub fn object(seed: u64) -> Generator<String> {
    Generator::<String>::new(&OBJECT_AUTOMATON, Box::new(PRandomizer::new(seed)))
}

/// Returns a JSON Mutator
///
/// # Examples
/// ```rust
/// use talasum::json;
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
pub fn mutate(input: &str, seed: u64) -> Option<Mutator> {
    Mutator::new::<JsonLexer, Rule>(
        Box::new(PRandomizer::new(seed)),
        input,
        Rule::value,
        Box::new(Config::new()),
    )
}
