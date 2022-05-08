use crate::configuration::Configurable;
use crate::randomness::Randomizer;
use crate::tokenizer::tokenize_input;
use crate::tokenizer::{AutomatonToken, LexerRule};
use pest::Parser;
use std::collections::{BTreeMap, BTreeSet};

/// A mutation-based fuzzer.
///
/// It is a seedable iterator which produces new versions of its
/// text input by understanding its structure and changing parts of it. The
/// degree of these changes depends on the horizontal and vertical fuzzing global coefficients.
///
/// It relies on a PRG internally because the fuzzing process should be traceable and reproducible at all times.
/// If one needs true randomness, one needs to generate truly random seeds to pass to one's
/// Mutator.
pub struct Mutator<'a> {
    seeder: Box<dyn Randomizer>,
    tokens: Vec<AutomatonToken<'a>>,
    input: &'a str,
    config: Box<dyn Configurable>,
}

impl <'a> Mutator<'a> {
    /// Creates a Mutator instance based on the following input:
    /// - `seeder` - will be used for generating random mutations to the input
    /// - `input` - valid input as per the protocol's specification
    /// - `rule` - name of the top rule of the corresponding PEG, usually R::value,
    /// where R and P are protocol-specific types defined in [`crate::tokenizer`]
    ///
    /// Result will be [`std::option::Option::None`] if the input is invalid as per the underlying
    /// protocol grammar, e.g. "{1}" is not a valid JSON input, hence cannot
    /// be fuzzed.
    pub(crate) fn new<P: Parser<R>, R: 'a + LexerRule>(
        seeder: Box<dyn Randomizer>,
        input: &'a str,
        rule: R,
        config: Box<dyn Configurable>,
    ) -> Option<Self> {
        tokenize_input::<'a, P, R>(input, rule).map(|tokens| Self {
            seeder,
            tokens,
            input,
            config,
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
        if offset != 0 {
            offset_table
                .entry(original)
                .and_modify(|e| *e += offset)
                .or_insert(offset);
        }
    }

    /// Chooses a set of tokens to be fuzzed.
    /// Outputs the indices of the tokens to be fuzzed in ascending order.
    fn choose_for_mutation(&self, seed: u64) -> BTreeSet<usize> {
        if self.tokens.is_empty() {
            BTreeSet::<usize>::new()
        } else {
            let final_cnt = self.get_tokens_count();
            let mut curr_idx = seed as usize % self.tokens.len();

            let mut chosen = BTreeSet::<usize>::new();
            for _ in 0..final_cnt {
                chosen.insert(curr_idx);

                // + 1 in order to avoid cycles
                curr_idx = (curr_idx + 1) % self.tokens.len();
            }
            chosen
        }
    }

    /// Computes the number of tokens to be fuzzed based on config.
    /// It is directly proportional to the horizontal fuzzing coefficient:
    /// If the H coefficient is set to max, every single token in the input
    /// will be fuzzed, effectively getting the behavior of a [`crate::Generator`].
    fn get_tokens_count(&self) -> usize {
        ((self.config.get_horizontal_randomness_coef() as f32 / 100_f32)
            * (self.tokens.len() as f32))
            .ceil() as usize
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

        let new_from = Self::get_moved_index(offsets, from);
        let new_to = Self::get_moved_index(offsets, to);

        if let Some(to_fuzz) = result.get(new_from..new_to) {
            let fuzzed = &automaton.traverse(String::from(to_fuzz), seed);
            result.replace_range(new_from..new_to, fuzzed);
            Self::move_index(
                offsets,
                to,
                fuzzed.len() as i64 - (new_to - new_from) as i64,
            );
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

impl <'a> Iterator for Mutator<'a> {
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
    use crate::configuration::{Config, Configurable, MockConfigurable};
    use crate::randomness::PRandomizer;
    use crate::tokenizer::json_lexer::{JsonLexer, Rule};
    use std::collections::BTreeMap;

    fn get_mutator_helper(seed: u64, input: &str) -> Mutator {
        Mutator::new::<JsonLexer, Rule>(
            Box::new(PRandomizer::new(seed)),
            input,
            Rule::value,
            Box::new(Config::new()),
        )
        .unwrap()
    }

    fn get_mocked_mutator_helper(
        seed: u64,
        input: &str,
        config: Box<dyn Configurable>,
    ) -> Mutator {
        Mutator::new::<JsonLexer, Rule>(
            Box::new(PRandomizer::new(seed)),
            input,
            Rule::value,
            config,
        )
        .unwrap()
    }

    #[test]
    fn empty_input_cannot_be_mutated() {
        assert_eq!(get_mutator_helper(1, "").next(), None);
    }

    #[test]
    fn mutators_require_valid_input() {
        assert!(Mutator::new::<JsonLexer, Rule>(
            Box::new(PRandomizer::new(1)),
            "(",
            Rule::value,
            Box::new(Config::new())
        )
        .is_none());
    }

    #[test]
    fn mutation_is_reproducible() {
        let mut first = get_mutator_helper(1, "123");
        let mut sec = get_mutator_helper(1, "123");
        assert_eq!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn mutation_is_seedable() {
        let mut first = get_mutator_helper(1, "1");
        let mut sec = get_mutator_helper(2, "123");
        assert_ne!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn mutation_produces_different_result_each_time() {
        let mut first = get_mutator_helper(1, "123");

        // search for a change in output in the first few values
        let mut different = false;
        for _ in 1..3 {
            if first.next().unwrap() != first.next().unwrap() {
                different = true;
                break;
            }
        }
        assert!(different);
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
    fn position_moves_get_summed_up() {
        let mut offsets = BTreeMap::<usize, i64>::new();
        Mutator::move_index(&mut offsets, 3, 2);
        Mutator::move_index(&mut offsets, 5, 2);
        assert_eq!(Mutator::get_moved_index(&offsets, 4), 6);
        assert_eq!(Mutator::get_moved_index(&offsets, 5), 7); // moves are not inclusive
        assert_eq!(Mutator::get_moved_index(&offsets, 6), 10);
    }

    #[test]
    fn automata_not_filtered_upon_max_quota_with_single_automaton() {
        let mutator = get_mutator_helper(123, "1234");
        assert_eq!(mutator.choose_for_mutation(0).len(), 1);
        assert_eq!(mutator.choose_for_mutation(1).len(), 1);
        assert_eq!(mutator.choose_for_mutation(2).len(), 1);
    }

    #[test]
    fn nothing_to_choose_for_mutation_when_input_is_empty() {
        let mutator = get_mutator_helper(123, "");
        assert_eq!(mutator.choose_for_mutation(0).len(), 0);
    }

    #[test]
    fn automata_are_chosen_in_asc_order() {
        let mutator = get_mutator_helper(123, "1234");
        let chosen = mutator.choose_for_mutation(100);
        assert!(chosen.len() > 0);
        for el_idx in 1..chosen.len() {
            assert!(chosen.get(&el_idx).unwrap() > chosen.get(&(el_idx - 1)).unwrap());
        }
    }

    #[test]
    fn automata_not_filtered_upon_max_quota_with_multiple_automata() {
        let mut mocked: MockConfigurable = MockConfigurable::new();
        mocked
            .expect_get_horizontal_randomness_coef()
            .return_const(100_u32);

        assert_eq!(
            get_mocked_mutator_helper(123, "[1,2,3]", Box::new(mocked))
                .choose_for_mutation(0)
                .len(),
            4
        );
    }

    #[test]
    #[should_panic]
    fn panic_when_fuzzing_token_with_invalid_idx() {
        get_mutator_helper(123, "[1,2,3]").fuzz_token(
            123,
            10000,
            &mut BTreeMap::<usize, i64>::new(),
            &mut String::new(),
        );
    }

    #[test]
    fn there_is_always_at_least_one_token_to_be_fuzzed() {
        let mut mocked: MockConfigurable = MockConfigurable::new();
        mocked
            .expect_get_horizontal_randomness_coef()
            .return_const(1_u32);

        assert_eq!(
            get_mocked_mutator_helper(123, "[1,2,3]", Box::new(mocked)).get_tokens_count(),
            1
        );
    }

    #[test]
    fn number_of_fuzzed_tokens_is_proportional_to_h_coef() {
        for (cnt, coef) in vec![
            (1, 20_u32),
            (2, 40_u32),
            (3, 60_u32),
            (4, 80_u32),
            (5, 100_u32),
        ] {
            let mut mocked: MockConfigurable = MockConfigurable::new();
            mocked
                .expect_get_horizontal_randomness_coef()
                .return_const(coef);

            assert_eq!(
                get_mocked_mutator_helper(123, "[1,2,3,4]", Box::new(mocked)).get_tokens_count(),
                cnt
            );
        }
    }

    #[test]
    fn mutating_different_inputs_produces_different_result() {
        let mut first = get_mutator_helper(1, "123");
        let mut sec = get_mutator_helper(1, "null");
        assert_ne!(first.next().unwrap(), sec.next().unwrap());
    }
}
