use crate::randomness::{PRandomizer, Randomizer};
pub mod helper;
pub mod json;
pub mod weights;

// Transformation is an operation to be performed on a value as it's moved
// through the automaton
type Transformation<T> = fn(T) -> T;

/// Transition from one state in the automaton to another one adjacent to it.
/// If the value is None, then the state is final and there isn't a state
/// to transition to.
type Transition<T> = Box<dyn Fn(u64) -> Option<&'static AutomatonNode<T>> + std::marker::Sync>;

/// In case of generation-based fuzzing an initial value is generated before
/// it's being sent for traversing.
/// This describes functions that generate initial values based on a seed.
type Generate<T> = fn(u64) -> T;

/// Magi Automaton states represent automaton states (graph nodes). Each is qualified
/// by a set of edges. Which edge is chosen depends on the seed's next value,
/// meaning decisions are retraceable (since they are based on pseudo-randomness).
pub struct AutomatonNode<T: 'static> {
    transition: Transition<T>,
    transformation: Transformation<T>,
}

/// Magi automatons are finite state machines with a predefined set of states and edges.
/// Each state represents an unfinished fuzz result. Each edge represents an operation
/// to be performed on said result. The decision of what is the next edge is performed by
/// the state decision function that takes a seed (int sequence) and based on it, it generates
/// a pseudo-random number - quota. The quota falls into one category out of multiple pre-defined
/// ones, each associated with an edge candidate.  
pub struct Automaton<T: 'static + Eq> {
    initial_node: &'static AutomatonNode<T>,
    generator: Generate<T>,
}

impl<T: Eq + core::fmt::Debug> Automaton<T> {
    /// Returns the start state for the automaton
    fn init_state(&self) -> &AutomatonNode<T> {
        self.initial_node
    }

    /// Returns the initial value to be fuzzed in case of generation-based fuzzing
    fn init_value(&self, seed: u64) -> T {
        (self.generator)(seed)
    }

    /// Generates an initial value and fuzzes it
    pub fn generate(&self, seed: u64) -> T {
        self.traverse(self.init_value(seed), seed)
    }

    /// Fuzzes the `input` value based on the `seed` value
    pub fn traverse(&self, input: T, seed: u64) -> T {
        // !TODO rename to mutate ?
        let mut seed = PRandomizer::new(seed);
        let mut value: T = input;
        let mut state: Option<&AutomatonNode<T>> = Some(self.init_state());
        let mut rand = seed.get();
        while let Some(AutomatonNode {
            transition,
            transformation,
        }) = state
        {
            value = transformation(value);
            state = transition(rand);
            rand = seed.get();
        }

        value
    }
}
