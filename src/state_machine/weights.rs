use super::{AutomatonNode1, Transition1};
use itertools::Itertools;

macro_rules! choose {
    ( $( ($w:expr, $x:expr) ),* ) => {
        {
            TransitionChoice::new(vec![
                $(
                    ($w, $x),
                )*
            ], 100).choose()
        }
    };
}
pub(crate) use choose;

pub struct TransitionChoice<T: 'static + Clone + Sync> {
    weights: Vec<WeightedTransition1<T>>,
}

// move this into the impl when stable: https://github.com/rust-lang/rust/issues/8995
type WeightedTransition1<T> = (u32, Option<&'static AutomatonNode1<T>>);

impl<T: 'static + Clone + Sync> TransitionChoice<T> {
    // const MAGIC_COEF: u32 = 100; // from 0 to 100

    // Magic == 0 is invalid and will break tests!!!
    pub fn new(mut weights: Vec<WeightedTransition1<T>>, magic: u32) -> Self {
        // sort the weights in ascending order, group into same numbers, multiply with 10*x, loop over and recalc
        weights.sort_by(|(w1, _), (w2, _)| w1.partial_cmp(&w2).unwrap());

        let mut prev_weight = 0;
        let mut recalculated = Vec::<WeightedTransition1<T>>::new();
        let mut top_limit = 0;

        for (weight, mut group) in &weights.into_iter().group_by(|(w, _)| *w) {
            let new_val = weight * magic + prev_weight * (100 - magic);
            prev_weight = weight;

            while let Some((_, tr)) = group.next() {
                top_limit += new_val;
                recalculated.push((top_limit, tr));
            }
        }
        Self {
            weights: recalculated,
        }
    }

    fn choice_func(&self, seed: u32) -> Option<&'static AutomatonNode1<T>> {
        if let Some(last) = self.weights.last() {
            let (last_weight, last_val) = last;
            let mut choice = *last_val;
            for (weight, value) in &self.weights {
                if *weight >= seed % last_weight {
                    choice = *value;
                    break;
                }
            }
            choice
        } else {
            None
        }
    }

    pub fn choose(self) -> Transition1<T> {
        Box::new(move |seed: u32| self.choice_func(seed))
    }
}

#[cfg(test)]
mod tests {
    use crate::state_machine::Transformation;

    use super::*;

    fn recalculate_helper(weights: Vec<u32>, magic: u32) -> Vec<u32> {
        TransitionChoice::<String>::new(weights.iter().map(|w| (*w, None)).collect_vec(), magic)
            .weights
            .iter()
            .map(|(w, _)| *w)
            .collect()
    }

    fn choose_helper(input: Vec<WeightedTransition1<String>>, seed: u32) -> Transformation<String> {
        TransitionChoice::<String>::new(input, 100).choose()(seed)
            .unwrap()
            .transformation
    }

    lazy_static! {
        static ref TEST_NODE1: AutomatonNode1<String> = AutomatonNode1::<String> {
            transition: Box::new(|_| None),
            transformation: |_| String::from("Test1"),
        };
        static ref TEST_NODE2: AutomatonNode1<String> = AutomatonNode1::<String> {
            transition: Box::new(|_| None),
            transformation: |_| String::from("Test2"),
        };
    }

    #[test]
    fn recalculation_for_empty_input_yields_none() {
        assert_eq!(recalculate_helper([].to_vec(), 0).len(), 0);
        assert_eq!(recalculate_helper([].to_vec(), 50).len(), 0);
        assert_eq!(recalculate_helper([].to_vec(), 100).len(), 0);
    }

    #[test]
    fn recalculation_for_single_transition() {
        for quota in [1, 50, 100] {
            assert_eq!(recalculate_helper([1].to_vec(), quota).len(), 1);
        }
    }

    #[test]
    fn recalculation_with_no_extremization_preserves_fractions() {
        let new_weights = recalculate_helper(vec![1, 2, 3], 100);
        assert_eq!(new_weights.len(), 3);
        assert_eq!(new_weights[0] * 2, new_weights[1] - new_weights[0]);
        assert_eq!(new_weights[0] * 3, new_weights[2] - new_weights[1]);
    }

    #[test]
    fn recalculation_with_equal_values_preserves_fractions() {
        for quota in [0, 50, 100] {
            let new_weights = recalculate_helper(vec![1, 1, 1], quota);
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
        for quota in [10, 50, 90] {
            let weights = recalculate_helper(vec![1, 2, 2, 3], quota);
            assert_eq!(weights.len(), 4);
            assert_eq!(weights[1] - weights[0], weights[2] - weights[1]);
            assert!(weights[1] - weights[0] > weights[0]);
            assert!(weights[3] - weights[2] > weights[2] - weights[1]);
        }
    }

    #[test]
    fn increasing_quota_increases_extremization() {
        let low_extremization = recalculate_helper(vec![1, 2], 90);
        let high_extremization = recalculate_helper(vec![1, 2], 10);
        assert!(
            (low_extremization[1] - low_extremization[0]) / low_extremization[0]
                < (high_extremization[1] - high_extremization[0]) / high_extremization[0]
        );
    }

    #[test]
    fn extremization_preserves_proportionality() {
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
            recalculated.weights[0].1.unwrap() as *const AutomatonNode1<String>,
            recalculated.weights[2].1.unwrap() as *const AutomatonNode1<String>
        );
        assert_eq!(
            recalculated.weights[1].1.unwrap() as *const AutomatonNode1<String>,
            recalculated.weights[3].1.unwrap() as *const AutomatonNode1<String>
        );
    }

    #[test]
    fn choose_with_no_transitions() {
        for quota in [1, 50, 80, 100] {
            for seed in [0, 1, 1000, 12312] {
                assert!(TransitionChoice::<String>::new(vec![], quota).choose()(seed).is_none());
            }
        }
    }

    #[test]
    fn choose_with_0_seed() {
        for quota in [1, 50, 80, 100] {
            assert!(
                TransitionChoice::<String>::new(vec![(1, Some(&TEST_NODE1))], quota).choose()(0)
                    .is_some()
            );
        }
    }

    #[test]
    fn choose_single_choice_regardless_seed() {
        // when there is only one choice, it should
        // be chosen regardless the seed
        for seed in [0, 100, 2000] {
            assert_eq!(
                choose_helper(vec![(1, Some(&TEST_NODE1))], seed)(String::new()),
                "Test1"
            );
        }
    }

    #[test]
    fn choose_correctly_from_multiple_choices_based_on_seed() {
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
        assert_eq!(
            (choose![(1, Some(&TEST_NODE1))](1234)
                .unwrap()
                .transformation)(String::new()),
            "Test1"
        );
    }
}
