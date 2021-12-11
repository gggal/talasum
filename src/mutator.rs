use crate::configuration::{Configurable, CONFIG};
use crate::randomness::Randomizer;
use crate::tokenizer::tokenize_input;
use crate::tokenizer::{AutomatonToken, LexerRule};
use pest::Parser;
use std::collections::{BTreeMap, HashSet};

/// A mutation-based fuzzer.
///
/// It is a seedable iterator which produces new versions of its
/// text input by understanding its structure and changing parts of it. The
/// degree of these changes depends on the horizontal and vertical global coefficients.
///
/// It relies on a PRG internally because the fuzzing process should be traceable and reproducible at all times.
/// If one needs true randomness, one needs to generate truly random seeds to pass to one's
/// Mutator.
pub struct Mutator {
    seeder: Box<dyn Randomizer>,
    tokens: Vec<AutomatonToken<'static>>,
    input: &'static str,
}

impl Mutator {
    /// Creates a Mutator instance based on the following input:
    /// - `seeder` - will be used for generating random mutations to the input
    /// - `input` - valid input as per the protocol's specification
    /// - `rule` - name of the top rule of the corresponding PEG, usually R::value,
    /// where R and P are protocol-specific types defined in [`crate::tokenizer`]
    ///
    /// Result will be [`std::option::Option::None`] if the input is invalid as per the underlying
    /// protocol grammar, e.g. "{1}" is not a valid JSON input, hence cannot
    /// be fuzzed.
    pub(crate) fn new<P: Parser<R>, R: 'static + LexerRule>(
        seeder: Box<dyn Randomizer>,
        input: &'static str,
        rule: R,
    ) -> Option<Self> {
        tokenize_input::<P, R>(input, rule).map(|tokens| Self {
            seeder,
            tokens,
            input,
        })
        // TODO ERROR log in case of invalid input
    }

    /// Calculates the new index of the element at `original`
    /// based on previous moves defined in `offset_table`.
    /// `offset_table` maps original indices in a sequence, 0,1,2... , to offsets to
    /// new indices after a series of changes to the sequence.
    ///
    /// For example, the pair 5 -> -2 means that the 5th element was, at some point,
    /// moved to index 3.
    fn get_moved_index(offset_table: &BTreeMap<usize, i64>, original: usize) -> usize {
        offset_table
            .range(0..original)
            .fold(original, |acc, (_, offset)| (acc as i64 + offset) as usize)
    }

    /// Reflects an index move of the element at `original` in the `offset_table`.
    /// `offset_table` maps original indices in a sequence, 0,1,2... , to offsets to
    /// new indices after a series of changes to the sequence.
    ///
    /// For example, the pair 5 -> -2 means that the 5th element was, at some point,
    /// moved to index 3.
    fn move_index(offset_table: &mut BTreeMap<usize, i64>, original: usize, offset: i64) {
        offset_table
            .entry(original)
            .and_modify(|e| *e += offset)
            .or_insert(offset);
    }

    /// Chooses a set of tokens to be fuzzed. The number of tokens to be
    /// fuzzed is directly proportional to the horizontal fuzzing coefficient:
    /// If the H coefficient is set to max, every single token in the input
    /// will be fuzzed, effectively simulating the behavior of a [`crate::Generator`].
    ///
    /// Outputs the indices of the tokens to be fuzzed.
    fn choose_for_mutation(&self, seed: u64) -> HashSet<usize> {
        if self.tokens.is_empty() {
            HashSet::<usize>::new()
        } else {
            let final_cnt =
                CONFIG.get_horizontal_randomness_coef() / 100 * (self.tokens.len() as u32);
            let mut curr_idx = seed as usize % self.tokens.len();

            let mut chosen = HashSet::<usize>::new();
            for _ in 0..final_cnt {
                chosen.insert(curr_idx);

                // + 1 in order to avoid cycles
                curr_idx = (curr_idx + 1) % self.tokens.len();
            }
            chosen
        }
    }

    /// Fuzzes the token at index `idx` using the `seed` value and
    /// updates the offset table and result value after.
    fn fuzz_token(
        &self,
        seed: u64,
        idx: usize,
        offsets: &mut BTreeMap<usize, i64>,
        result: &mut String,
    ) {
        let AutomatonToken {
            from,
            to,
            automaton,
        } = self.tokens[idx];
        if let Some(to_fuzz) = self.input.get(from..to) {
            let fuzzed = &automaton.traverse(String::from(to_fuzz), seed);
            result.replace_range(
                Self::get_moved_index(offsets, from)..Self::get_moved_index(offsets, to),
                fuzzed,
            );
            Self::move_index(offsets, to, fuzzed.len() as i64 - to as i64);
        } else {
            panic!("Unreachable!");
        }
    }

    /// Fuzzes the whole input
    fn fuzz(&mut self) -> String {
        let next_seed = self.seeder.get();
        let mut offsets = BTreeMap::<usize, i64>::new();
        let mut result = String::from(self.input);

        for idx in self.choose_for_mutation(next_seed) {
            self.fuzz_token(next_seed, idx, &mut offsets, &mut result);
        }
        result
    }
}

impl Iterator for Mutator {
    type Item = String;

    /// Computes a new fuzz value.
    ///
    /// Returns `None` if the input doesn't contain
    /// any tokens, e.g. an empty string.
    fn next(&mut self) -> Option<Self::Item> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(self.fuzz())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Mutator;
    use crate::randomness::PRandomizer;
    use crate::tokenizer::json_lexer::{JsonLexer, Rule};
    use std::collections::BTreeMap;

    fn valid_mutator() -> Mutator {
        Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(123)), "[1,2,3]", Rule::value)
            .unwrap()
    }

    #[test]
    fn empty_input_cannot_be_mutated() {
        assert_eq!(
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(1)), "", Rule::value)
                .unwrap()
                .next(),
            None
        );
    }

    #[test]
    fn mutators_require_valid_input() {
        assert!(
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(1)), "(", Rule::value)
                .is_none()
        );
    }

    #[test]
    fn mutation_is_reproducible() {
        let mut first =
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(1)), "123", Rule::value)
                .unwrap();
        let mut sec =
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(1)), "123", Rule::value)
                .unwrap();
        assert_eq!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn mutation_is_seedable() {
        let mut first =
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(1)), "1", Rule::value)
                .unwrap();
        let mut sec =
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(2)), "1", Rule::value)
                .unwrap();
        assert_ne!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn mutation_different_inputs_produces_different_result() {
        let mut first =
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(1)), "123", Rule::value)
                .unwrap();
        let mut sec =
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(1)), "124", Rule::value)
                .unwrap();
        assert_ne!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn mutation_produces_different_result_each_time() {
        let mut first =
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(1)), "123", Rule::value)
                .unwrap();
        assert_ne!(first.next().unwrap(), first.next().unwrap());
    }

    #[test]
    fn get_moved_index_with_no_offset_table() {
        assert_eq!(
            Mutator::get_moved_index(&BTreeMap::<usize, i64>::new(), 1234),
            1234
        );
        assert_eq!(
            Mutator::get_moved_index(&BTreeMap::<usize, i64>::new(), 0),
            0
        );
        assert_eq!(
            Mutator::get_moved_index(&BTreeMap::<usize, i64>::new(), 1),
            1
        );
    }

    #[test]
    fn get_moved_index_with_positive_offsets() {
        let mut offsets = BTreeMap::<usize, i64>::new();
        offsets.insert(4, 5);
        assert_eq!(Mutator::get_moved_index(&offsets, 5), 10);
    }

    #[test]
    fn moved_positions_are_not_inclusive() {
        let mut offsets = BTreeMap::<usize, i64>::new();
        offsets.insert(5, 5);
        assert_ne!(Mutator::get_moved_index(&offsets, 5), 10);
    }

    #[test]
    fn get_moved_index_with_negative_offsets() {
        let mut offsets = BTreeMap::<usize, i64>::new();
        offsets.insert(4, -2);
        assert_eq!(Mutator::get_moved_index(&offsets, 5), 3);
    }

    #[test]
    fn get_moved_index_after_multiple_moves() {
        let mut offsets = BTreeMap::<usize, i64>::new();
        offsets.insert(4, 5);
        offsets.insert(5, 7);
        offsets.insert(7, -2);
        assert_eq!(Mutator::get_moved_index(&offsets, 8), 18);
    }

    #[test]
    fn move_index_when_offset_table_is_empty() {
        let mut offsets = BTreeMap::<usize, i64>::new();
        Mutator::move_index(&mut offsets, 2, 3);
        assert!(offsets.contains_key(&2));
        assert_eq!(offsets.get(&2).unwrap(), &3);
    }

    #[test]
    fn move_same_position_repeatedly() {
        let mut offsets = BTreeMap::<usize, i64>::new();
        Mutator::move_index(&mut offsets, 2, 3);
        Mutator::move_index(&mut offsets, 2, 5);
        Mutator::move_index(&mut offsets, 2, -2);
        assert!(offsets.contains_key(&2));
        assert_eq!(offsets.get(&2).unwrap(), &6);
    }

    #[test]
    fn automata_not_filtered_upon_max_quota_with_single_automaton() {
        let mutator =
            Mutator::new::<JsonLexer, Rule>(Box::new(PRandomizer::new(123)), "1234", Rule::value)
                .unwrap();
        assert_eq!(mutator.choose_for_mutation(0).len(), 1);
        assert_eq!(mutator.choose_for_mutation(1).len(), 1);
        assert_eq!(mutator.choose_for_mutation(2).len(), 1);
    }

    #[test]
    fn automata_not_filtered_upon_max_quota_with_multiple_automata() {
        assert_eq!(valid_mutator().choose_for_mutation(0).len(), 4);
    }

    #[test]
    #[should_panic]
    fn panic_when_fuzzing_token_with_invalid_idx() {
        valid_mutator().fuzz_token(
            123,
            10000,
            &mut BTreeMap::<usize, i64>::new(),
            &mut String::new(),
        );
    }
}
