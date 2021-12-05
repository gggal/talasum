use crate::configuration::{Configurable, CONFIG};
use crate::randomization::prandomizer::PRandomizer;
use crate::tokenizer::tokenize_input;
use crate::tokenizer::{AutomatonToken, LexerRule};
use pest::Parser;
use std::collections::{BTreeMap, HashSet};

pub struct Mutator {
    seeder: PRandomizer,
    tokens: Vec<AutomatonToken<'static>>,
    input: &'static str,
}

impl Mutator {

    pub(crate) fn new<P: Parser<R>, R: 'static + LexerRule>(
        seed: u32,
        input: &'static str,
        rule: R,
    ) -> Option<Self> {
        tokenize_input::<P, R>(input, rule).map(|tokens| Self {
            seeder: PRandomizer::new(seed as u64),
            tokens,
            input,
        })
        // TODO ERROR log in case of invalid input
    }

    fn get_moved_index(offset_table: &BTreeMap<u32, i32>, original: u32) -> i32 {
        original as i32
            + offset_table
                .range(0..original)
                .fold(0, |acc, (_, offset)| acc as i32 + offset)
    }

    fn move_index(offset_table: &mut BTreeMap<u32, i32>, original: u32, offset: i32) {
        offset_table
            .entry(original)
            .and_modify(|e| *e += offset)
            .or_insert(offset);
    }

    fn choose_for_mutation(&self, seed: u32) -> HashSet<u32> {
        if self.tokens.is_empty() {
            HashSet::<u32>::new()
        } else {
            let final_cnt =
                CONFIG.get_horizontal_randomness_coef() / 100 * (self.tokens.len() as u32);
            let mut curr_idx = seed % self.tokens.len() as u32;

            let mut chosen = HashSet::<u32>::new();
            for _ in 0..final_cnt {
                chosen.insert(curr_idx);

                // + 1 in order to avoid cycles
                curr_idx = (curr_idx + 1) % self.tokens.len() as u32;
            }
            chosen
        }
    }

    fn fuzz_token(&self, seed: u32, idx: u32, offsets: &mut BTreeMap::<u32, i32>, result: &mut String) {
        let AutomatonToken {
            from,
            to,
            automaton,
        } = self.tokens[idx as usize];
        if let Some(to_fuzz) = self.input.get(from as usize..to as usize) {
            let fuzzed = &automaton.traverse(String::from(to_fuzz), seed);
            result.replace_range(
                Self::get_moved_index(&offsets, from) as usize
                    ..Self::get_moved_index(&offsets, to) as usize,
                fuzzed,
            );
            Self::move_index(offsets, to, fuzzed.len() as i32 - to as i32);
        } else {
            panic!("Unreachable!");
        }
    }

    fn fuzz(&mut self) -> String {
        let next_seed = self.seeder.get();
        let mut offsets = BTreeMap::<u32, i32>::new();
        let mut result = String::from(self.input);

        for idx in self.choose_for_mutation(next_seed) {
            self.fuzz_token(next_seed, idx, &mut offsets, &mut result);
        }
        result
    }
}

impl Iterator for Mutator {
    type Item = String;

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
    use crate::tokenizer::json_lexer::{JsonLexer, Rule};
    use std::collections::BTreeMap;

    #[test]
    fn empty_input_cannot_be_mutated() {
        assert_eq!(
            Mutator::new::<JsonLexer, Rule>(1, "", Rule::value)
                .unwrap()
                .next(),
            None
        );
    }

    #[test]
    fn mutators_require_valid_input() {
        assert!(Mutator::new::<JsonLexer, Rule>(1, "(", Rule::value).is_none());
    }

    #[test]
    fn mutation_is_reproducible() {
        let mut first = Mutator::new::<JsonLexer, Rule>(1, "123", Rule::value).unwrap();
        let mut sec = Mutator::new::<JsonLexer, Rule>(1, "123", Rule::value).unwrap();
        assert_eq!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn mutation_is_seedable() {
        let mut first = Mutator::new::<JsonLexer, Rule>(1, "1", Rule::value).unwrap();
        let mut sec = Mutator::new::<JsonLexer, Rule>(2, "1", Rule::value).unwrap();
        assert_ne!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn mutation_different_inputs_produces_different_result() {
        let mut first = Mutator::new::<JsonLexer, Rule>(1, "123", Rule::value).unwrap();
        let mut sec = Mutator::new::<JsonLexer, Rule>(1, "124", Rule::value).unwrap();
        assert_ne!(first.next().unwrap(), sec.next().unwrap());
    }

    #[test]
    fn mutation_produces_different_result_each_time() {
        let mut first = Mutator::new::<JsonLexer, Rule>(1, "123", Rule::value).unwrap();
        assert_ne!(first.next().unwrap(), first.next().unwrap());
    }

    #[test]
    fn get_moved_index_with_no_offset_table() {
        assert_eq!(
            Mutator::get_moved_index(&BTreeMap::<u32, i32>::new(), 1234),
            1234
        );
        assert_eq!(Mutator::get_moved_index(&BTreeMap::<u32, i32>::new(), 0), 0);
        assert_eq!(Mutator::get_moved_index(&BTreeMap::<u32, i32>::new(), 1), 1);
    }

    #[test]
    fn get_moved_index_with_positive_offsets() {
        let mut offsets = BTreeMap::<u32, i32>::new();
        offsets.insert(4, 5);
        assert_eq!(Mutator::get_moved_index(&offsets, 5), 10);
    }

    #[test]
    fn moved_positions_are_not_inclusive() {
        let mut offsets = BTreeMap::<u32, i32>::new();
        offsets.insert(5, 5);
        assert_ne!(Mutator::get_moved_index(&offsets, 5), 10);
    }

    #[test]
    fn get_moved_index_with_negative_offsets() {
        let mut offsets = BTreeMap::<u32, i32>::new();
        offsets.insert(4, -2);
        assert_eq!(Mutator::get_moved_index(&offsets, 5), 3);
    }

    #[test]
    fn get_moved_index_after_multiple_moves() {
        let mut offsets = BTreeMap::<u32, i32>::new();
        offsets.insert(4, 5);
        offsets.insert(5, 7);
        offsets.insert(7, -2);
        assert_eq!(Mutator::get_moved_index(&offsets, 8), 18);
    }

    #[test]
    fn move_index_when_offset_table_is_empty() {
        let mut offsets = BTreeMap::<u32, i32>::new();
        Mutator::move_index(&mut offsets, 2, 3);
        assert!(offsets.contains_key(&2));
        assert_eq!(offsets.get(&2).unwrap(), &3);
    }

    #[test]
    fn move_same_position_repeatedly() {
        let mut offsets = BTreeMap::<u32, i32>::new();
        Mutator::move_index(&mut offsets, 2, 3);
        Mutator::move_index(&mut offsets, 2, 5);
        Mutator::move_index(&mut offsets, 2, -2);
        assert!(offsets.contains_key(&2));
        assert_eq!(offsets.get(&2).unwrap(), &6);
    }

    #[test]
    fn automata_not_filtered_upon_max_quota_with_single_automaton() {
        let mutator = Mutator::new::<JsonLexer, Rule>(123, "1234", Rule::value).unwrap();
        assert_eq!(mutator.choose_for_mutation(0).len(), 1);
        assert_eq!(mutator.choose_for_mutation(1).len(), 1);
        assert_eq!(mutator.choose_for_mutation(2).len(), 1);
    }

    #[test]
    fn automata_not_filtered_upon_max_quota_with_multiple_automata() {
        let mutator = Mutator::new::<JsonLexer, Rule>(123, "[1,2,3]", Rule::value).unwrap();
        assert_eq!(mutator.choose_for_mutation(0).len(), 4);
    }
}
