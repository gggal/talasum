use super::{AutomatonNode, Transition};
use crate::configuration::{Config, Configurable};
use itertools::Itertools;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

/// The entrypoint to this module.
/// Based on a given set or transitions, recalculates their weights
/// and returns a function that accepts a seed and returns the next state
pub fn choose(vec: Vec<WeightedTransition<String>>) -> Transition<String> {
    TransitionChoice::<String>::new(vec, CONFIG.get_vertical_randomness_coef()).choose()
}

/// Represents the transition function.
/// It is a function of the seed, weights and v_coef:
///
/// - the seed is expected to vary for each run in order to guarantee diverse
/// output values
/// - the weights are the predefined proportions at which each transition is
/// likely to happen
/// - v_coef is a configurable value which determines the degree
/// of the extremeness of the output
///
/// The output of the function is another automaton state (or
/// no state if the current state is final).
pub struct TransitionChoice<T: 'static + Clone + Sync> {
    weights: Vec<WeightedTransition<T>>,
}

/// Represents an edge in a weighted automaton. i.e. the potential transition
/// from one state of the automaton onto another based on some proportional
/// probability.
/// The transition can lead nowhere in which case the current state is final.
/// The transition can also redirect the current state to itself but this may
/// lead to cycles and must be avoided.
/// The weight is represented by a single unsigned number and only makes sense
/// in the context of the weights of the rest of the transitions from the
/// current state to any other state. In that context each weight m is m/n'th
/// where n is the sum of all weights for transitions from the current state.
type WeightedTransition<T> = (u32, Option<&'static AutomatonNode<T>>);

impl<T: 'static + Clone + Sync> TransitionChoice<T> {
    // TODO move WeightedTransition into the impl when stable: https://github.com/rust-lang/rust/issues/8995

    pub fn new(mut weights: Vec<WeightedTransition<T>>, v_coef: u32) -> Self {
        weights.sort_by(|(w1, _), (w2, _)| w1.partial_cmp(w2).unwrap());

        let mut prev_weight = 0;
        let mut recalculated = Vec::<WeightedTransition<T>>::new();
        let mut top_limit = 0;

        for (weight, group) in &weights.into_iter().group_by(|(w, _)| *w) {
            let new_val = weight * v_coef + prev_weight * (100 - v_coef);
            prev_weight = weight;

            for (_, tr) in group {
                top_limit += new_val;
                recalculated.push((top_limit, tr));
            }
        }
        Self {
            weights: recalculated,
        }
    }

    /// Chooses the next state based on the seed and the recalculated
    /// weights of the reachable states.
    fn choice_func(&self, seed: u64) -> Option<&'static AutomatonNode<T>> {
        if let Some(last) = self.weights.last() {
            let (last_weight, last_val) = last;
            let mut choice = *last_val;
            for (weight, value) in &self.weights {
                if *weight >= seed as u32 % last_weight {
                    choice = *value;
                    break;
                }
            }
            choice
        } else {
            None
        }
    }

    /// Returns an instance of the choice function
    pub fn choose(self) -> Transition<T> {
        Box::new(move |seed: u64| self.choice_func(seed))
    }
}

#[cfg(test)]
mod tests {
    use crate::state_machine::Transformation;

    use super::*;

    fn recalculate_helper(weights: Vec<u32>, v_coef: u32) -> Vec<u32> {
        TransitionChoice::<String>::new(weights.iter().map(|w| (*w, None)).collect_vec(), v_coef)
            .weights
            .iter()
            .map(|(w, _)| *w)
            .collect()
    }

    fn choose_helper(input: Vec<WeightedTransition<String>>, seed: u64) -> Transformation<String> {
        // choose with v-randomness set at max
        TransitionChoice::<String>::new(input, 100).choose()(seed)
            .unwrap()
            .transformation
    }

    lazy_static! {
        static ref TEST_NODE1: AutomatonNode<String> =
            AutomatonNode::<String>::new().set_func(|_| String::from("Test1"));
        static ref TEST_NODE2: AutomatonNode<String> =
            AutomatonNode::<String>::new().set_func(|_| String::from("Test2"));
    }

    #[test]
    fn recalculation_for_empty_input_yields_none() {
        assert_eq!(recalculate_helper([].to_vec(), 0).len(), 0);
        assert_eq!(recalculate_helper([].to_vec(), 50).len(), 0);
        assert_eq!(recalculate_helper([].to_vec(), 100).len(), 0);
    }

    #[test]
    fn recalculation_for_single_transition() {
        for v_coef in [1, 50, 100] {
            assert_eq!(recalculate_helper([1].to_vec(), v_coef).len(), 1);
        }
    }

    #[test]
    fn recalculation_with_min_v_randomness_preserves_fractions() {
        let new_weights = recalculate_helper(vec![1, 2, 3], 1);
        assert_eq!(new_weights.len(), 3);
        assert_eq!(new_weights[0] * 101, new_weights[1] - new_weights[0]);
        assert_eq!(new_weights[0] * 201, new_weights[2] - new_weights[1]);
    }

    #[test]
    fn recalculation_with_max_v_randomness_preserves_fractions() {
        let new_weights = recalculate_helper(vec![1, 2, 3], 100);
        assert_eq!(new_weights.len(), 3);
        assert_eq!(new_weights[0] * 2, new_weights[1] - new_weights[0]);
        assert_eq!(new_weights[0] * 3, new_weights[2] - new_weights[1]);
    }

    #[test]
    fn recalculation_with_equal_values_preserves_fractions() {
        for v_coef in [1, 50, 100] {
            let new_weights = recalculate_helper(vec![1, 1, 1], v_coef);
            assert_eq!(new_weights.len(), 3);
            assert_eq!(new_weights[0] * 2, new_weights[1]);
            assert_eq!(new_weights[0] * 3, new_weights[2]);
        }
    }

    #[test]
    fn reverse_sorted_list_yields_same_result() {
        let reverse_sorted = recalculate_helper(vec![3, 2, 1], 50);
        let sorted = recalculate_helper(vec![1, 2, 3], 50);
        assert_eq!(reverse_sorted.len(), sorted.len());
        assert_eq!(reverse_sorted[0], sorted[0]);
        assert_eq!(reverse_sorted[1], sorted[1]);
        assert_eq!(reverse_sorted[2], sorted[2]);
    }

    #[test]
    fn unsorted_list_yields_same_result() {
        let unsorted = recalculate_helper(vec![2, 3, 1], 50);
        let sorted = recalculate_helper(vec![1, 2, 3], 50);
        assert_eq!(unsorted.len(), sorted.len());
        assert_eq!(unsorted[0], sorted[0]);
        assert_eq!(unsorted[1], sorted[1]);
        assert_eq!(unsorted[2], sorted[2]);
    }

    #[test]
    fn duplicate_elements_have_the_same_fraction() {
        for v_coef in [10, 50, 90] {
            let weights = recalculate_helper(vec![1, 2, 2, 3], v_coef);
            assert_eq!(weights.len(), 4);
            assert_eq!(weights[1] - weights[0], weights[2] - weights[1]);
            assert!(weights[1] - weights[0] > weights[0]);
            assert!(weights[3] - weights[2] > weights[2] - weights[1]);
        }
    }

    #[test]
    fn increasing_v_coef_increases_randomness() {
        let high_randomness = recalculate_helper(vec![1, 2], 90);
        let low_randomness = recalculate_helper(vec![1, 2], 10);
        assert!(
            (low_randomness[1] - low_randomness[0]) / low_randomness[0]
                > (high_randomness[1] - high_randomness[0]) / high_randomness[0]
        );
    }

    #[test]
    fn randomness_preserves_proportionality() {
        let small_proportion = recalculate_helper(vec![1, 2], 50);
        let big_proportion = recalculate_helper(vec![1, 100], 50);

        assert!(
            (small_proportion[1] - small_proportion[0]) / small_proportion[0]
                < (big_proportion[1] - big_proportion[0]) / big_proportion[0]
        );
    }

    #[test]
    fn proper_values_after_recalculation() {
        let recalculated = TransitionChoice::<String>::new(
            vec![
                (1, Some(&TEST_NODE1)),
                (2, Some(&TEST_NODE2)),
                (3, Some(&TEST_NODE1)),
                (4, Some(&TEST_NODE2)),
            ],
            85,
        );
        assert_eq!(
            recalculated.weights[0].1.unwrap() as *const AutomatonNode<String>,
            recalculated.weights[2].1.unwrap() as *const AutomatonNode<String>
        );
        assert_eq!(
            recalculated.weights[1].1.unwrap() as *const AutomatonNode<String>,
            recalculated.weights[3].1.unwrap() as *const AutomatonNode<String>
        );
    }

    #[test]
    fn choose_with_no_transitions() {
        for v_coef in [1, 50, 80, 100] {
            for seed in [0, 1, 1000, 12312] {
                assert!(TransitionChoice::<String>::new(vec![], v_coef).choose()(seed).is_none());
            }
        }
    }

    #[test]
    fn choose_with_0_seed() {
        for v_coef in [1, 50, 80, 100] {
            assert!(
                TransitionChoice::<String>::new(vec![(1, Some(&TEST_NODE1))], v_coef).choose()(0)
                    .is_some()
            );
        }
    }

    #[test]
    fn choose_single_choice_regardless_seed() {
        // when there is only one option, it should
        // be chosen regardless the seed
        for seed in [0, 100, 2000] {
            assert_eq!(
                choose_helper(vec![(1, Some(&TEST_NODE1))], seed)(String::new()),
                "Test1"
            );
        }
    }

    #[test]
    fn choose_correctly_from_multiple_options_based_on_seed() {
        for seed in [0, 1, 99, 100, 301] {
            assert_eq!(
                choose_helper(vec![(1, Some(&TEST_NODE1)), (2, Some(&TEST_NODE2))], seed)(
                    String::new()
                ),
                "Test1"
            );
        }

        for seed in [101, 299, 401, 599] {
            assert_eq!(
                choose_helper(vec![(1, Some(&TEST_NODE1)), (2, Some(&TEST_NODE2))], seed)(
                    String::new()
                ),
                "Test2"
            );
        }
    }

    #[test]
    fn choose_macro_expands_correctly() {
        let func = choose(vec![(1, Some(&TEST_NODE1))])(1234)
            .unwrap()
            .transformation;
        assert_eq!(func(String::new()), "Test1");
    }
}
