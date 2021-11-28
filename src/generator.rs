use crate::randomization::prandomizer::PRandomizer;
use crate::state_machine::Automaton;

pub struct Generator<T: 'static + Eq> {
    automaton: &'static Automaton<T>,
    seeder: PRandomizer,
}

impl<T: Eq> Generator<T> {
    pub fn new(automaton: &'static Automaton<T>, seed: u32) -> Self {
        Self {
            automaton: automaton,
            seeder: PRandomizer::new(seed as u64),
        }
    }
}

impl<T: Eq + core::fmt::Debug> Iterator for Generator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.automaton.generate(self.seeder.get()))
    }
}

#[cfg(test)]
mod tests {
    use super::Generator;
    use crate::state_machine::json::number::NUMBER_AUTOMATON;

    #[test]
    fn generation_is_reproducible() {
        let mut first = Generator::new(&NUMBER_AUTOMATON, 1);
        let mut sec = Generator::new(&NUMBER_AUTOMATON, 1);
        assert_eq!(first.next().unwrap(), sec.next().unwrap());
        assert_eq!(first.next().unwrap(), sec.next().unwrap());
        assert_eq!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn generation_is_seedable() {
        let mut first = Generator::new(&NUMBER_AUTOMATON, 1);
        let mut sec = Generator::new(&NUMBER_AUTOMATON, 2);
        assert_ne!(first.next().unwrap(), sec.next().unwrap());
    }
}
