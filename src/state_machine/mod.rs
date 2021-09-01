// use crate::state_machine::Basic;
use rand::thread_rng;
use rand::RngCore;
pub mod basic;
pub mod json;
use super::png;

/// To be implemented by every automaton.
///
/// Magi automatons are finite state machines with a predefined set of states and edges.
/// Each state represents an unfinished fuzz result. Each edge represents an operation
/// to be performed on said result. The decision of what is the next edge is performed by
/// the state decision function that takes a seed (int sequence) and based on it, it generates
/// a pseudo-random number - quota. The quota falls into one category out of multiple pre-defined
/// ones, each associated with an edge candidate.  
pub trait Automaton<T: Eq> {
    // To be provided by implementors!
    // Returns the initial value passed to the automaton
    fn init_value(&self) -> T;

    // To be provided by implementors!
    // Returns the initial state for the automaton
    fn init_state(&self) -> Box<dyn AutomatonState<T>>;

    fn seed(&self) -> Box<dyn RngCore> {
        Box::new(thread_rng())
    }

    // Traverses the graph and computes the end value
    fn traverse(&mut self) -> T {
        let mut value: T = self.init_value();
        let mut state: Box<dyn AutomatonState<T>> = self.init_state();
        let mut seed: Box<dyn RngCore> = self.seed();

        let mut rand = seed.as_mut().next_u32();
        while let Some((new_state, func)) = state.decide_next(rand) {
            value = func(rand, value);
            state = new_state;
            rand = seed.as_mut().next_u32();
        }

        value
    }
}

// Magi Automaton states represent automaton states (graph nodes). Each is qualified
// by a set of edges. Which edge is chosen depends on the seed's next value,
// meaning decisions are retraceble (since they are based on pseudo-randomness).
trait AutomatonState<T> {
    // Chooses the next path, based on the seed
    fn decide_next(&self, seed: u32) -> Option<AutomatonEdge<T>>;
}

// Transition is an operation to be performed on a value, as it's moved
// through the automaton
type Transition<T> = fn(u32, T) -> T;

// An automaton edge consists of next state and a transition
type AutomatonEdge<T> = (Box<dyn AutomatonState<T>>, Transition<T>);
