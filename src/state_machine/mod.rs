use crate::randomness::{PRandomizer, Randomizer};
pub mod json;
pub mod weights;
pub mod helper;

// Transition is an operation to be performed on a value, as it's moved
// through the automaton
type Transformation<T> = fn(T) -> T; // TODO maybe rename to mutate

type Transition<T> = Box<dyn Fn(u64) -> Option<&'static AutomatonNode<T>> + std::marker::Sync>;

type Generate<T> = fn(u64) -> T;

#[allow(dead_code)]

/// Magi Automaton states represent automaton states (graph nodes). Each is qualified
/// by a set of edges. Which edge is chosen depends on the seed's next value,
/// meaning decisions are retraceable (since they are based on pseudo-randomness).
pub struct AutomatonNode<T: 'static> {
    transition: Transition<T>,
    transformation: Transformation<T>,
}

#[allow(dead_code)]
pub struct Automaton<T: 'static + Eq> {
    initial_node: &'static AutomatonNode<T>,
    generator: Generate<T>,
}

/// Magi automatons are finite state machines with a predefined set of states and edges.
/// Each state represents an unfinished fuzz result. Each edge represents an operation
/// to be performed on said result. The decision of what is the next edge is performed by
/// the state decision function that takes a seed (int sequence) and based on it, it generates
/// a pseudo-random number - quota. The quota falls into one category out of multiple pre-defined
/// ones, each associated with an edge candidate.  
impl<T: Eq + core::fmt::Debug> Automaton<T> {
    // TODO might rename to FuzzEngine/FuzzAutomaton
    // Returns the initial state for the automaton
    fn init_state(&self) -> &AutomatonNode<T> {
        self.initial_node
    }

    fn init_value(&self, seed: u64) -> T {
        (self.generator)(seed)
    }

    pub fn generate(&self, seed: u64) -> T {
        self.traverse(self.init_value(seed), seed)
    }

    // Traverses the graph and computes the end value
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