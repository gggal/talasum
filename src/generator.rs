use crate::randomness::Randomizer;
use crate::state_machine::Automaton;

/// A generation-based fuzzer.
///
/// It is a seedable iterator which produces new versions of the
/// selected protocol<->token pair, for example, the protocol of choice might be
/// JSON and the token might be any of the valid JSON data types (number, string
/// and so on). The output for a JSON number will be significantly different that
/// the output for a JSON string because each of these are being represented by
/// a different token internally. The thoroughness of the fuzzing process depends
/// on the vertical fuzzing global coefficient.
///
/// It relies on a PRG internally because the fuzzing process should be traceable
/// and reproducible at all times. If one needs true randomness, one needs to
/// generate truly random seeds to pass to one's Generator.
pub struct Generator<T: 'static + Eq> {
    automaton: &'static Automaton<T>,
    seeder: Box<dyn Randomizer>,
}

impl<T: Eq> Generator<T> {
    /// Creates a Generator instance based on the following input:
    /// - `automaton` - automaton static object, representing the protocol<->type pair
    /// - `seeder` - will be used for generating random mutations to the input
    /// where R and P are protocol-specific types defined in [`crate::tokenizer`]
    pub fn new(automaton: &'static Automaton<T>, seeder: Box<dyn Randomizer>) -> Self {
        Self { automaton, seeder }
    }
}

impl<T: Eq + core::fmt::Debug> Iterator for Generator<T> {
    type Item = T;

    /// Computes a new fuzz value.
    ///
    /// Should never return `None`
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.automaton.generate(self.seeder.get()))
    }
}

#[cfg(test)]
mod tests {
    use super::Generator;
    use crate::randomness::PRandomizer;
    use crate::state_machine::json::number::NUMBER_AUTOMATON;

    #[test]
    fn generation_is_reproducible() {
        let mut first = Generator::new(&NUMBER_AUTOMATON, Box::new(PRandomizer::new(1)));
        let mut sec = Generator::new(&NUMBER_AUTOMATON, Box::new(PRandomizer::new(1)));
        assert_eq!(first.next().unwrap(), sec.next().unwrap());
        assert_eq!(first.next().unwrap(), sec.next().unwrap());
        assert_eq!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn generation_is_seedable() {
        let mut first = Generator::new(&NUMBER_AUTOMATON, Box::new(PRandomizer::new(1)));
        let mut sec = Generator::new(&NUMBER_AUTOMATON, Box::new(PRandomizer::new(2)));
        assert_ne!(first.next().unwrap(), sec.next().unwrap());
    }
}
