// use crate::state_machine::Basic;
use rand::thread_rng;
use rand::RngCore;
pub mod json;
pub mod weights;

// Transition is an operation to be performed on a value, as it's moved
// through the automaton
type Transformation<T> = fn(T) -> T; // TODO maybe rename to mutate

type Transition<'a, T> = fn(u32) -> Option<&'a AutomatonNode<'a, T>>;

type Transition1<T> = Box<dyn Fn(u32) -> Option<&'static AutomatonNode1<T>> + std::marker::Sync>;

type Generate<T> = fn(u32) -> T;

#[allow(dead_code)]

/// Magi Automaton states represent automaton states (graph nodes). Each is qualified
/// by a set of edges. Which edge is chosen depends on the seed's next value,
/// meaning decisions are retraceble (since they are based on pseudo-randomness).
struct AutomatonNode<'a, T: 'static> {
    transition: Transition<'a, T>,
    transformation: Transformation<T>,
}

pub struct AutomatonNode1<T: 'static> {
    transition: Transition1<T>,
    transformation: Transformation<T>,
}

/////
#[allow(dead_code)]
pub struct Automaton<'a, T: 'static + Eq> {
    initial_node: &'a AutomatonNode<'a, T>,
    generator: Generate<T>,
}

#[allow(dead_code)]
pub struct Automaton1<T: 'static + Eq> {
    initial_node: &'static AutomatonNode1<T>,
    generator: Generate<T>,
}

/// Magi automatons are finite state machines with a predefined set of states and edges.
/// Each state represents an unfinished fuzz result. Each edge represents an operation
/// to be performed on said result. The decision of what is the next edge is performed by
/// the state decision function that takes a seed (int sequence) and based on it, it generates
/// a pseudo-random number - quota. The quota falls into one category out of multiple pre-defined
/// ones, each associated with an edge candidate.  
impl<'a, T: Eq> Automaton<'a, T> {
    // TODO might rename to FuzzEngine/FuzzAutomaton
    // Returns the initial state for the automaton
    fn init_state(&self) -> &'a AutomatonNode<'a, T> {
        self.initial_node
    }

    fn init_value(&self, seed: u32) -> T {
        (self.generator)(seed)
    }

    // TODO delete this method
    fn seed(&self) -> Box<dyn RngCore> {
        Box::new(thread_rng())
    }

    fn generate(&self) -> T {
        let mut seed: Box<dyn RngCore> = self.seed();

        self.traverse(self.init_value(seed.as_mut().next_u32()))
    }

    // Traverses the graph and computes the end value
    fn traverse(&self, input: T) -> T {
        // !TODO rename to mutate ?
        let mut seed: Box<dyn RngCore> = self.seed();

        let mut value: T = input;
        let mut state: Option<&'a AutomatonNode<'a, T>> = Some(self.init_state());
        let mut rand = seed.as_mut().next_u32();

        while let Some(AutomatonNode {
            transition,
            transformation,
        }) = state
        {
            value = transformation(value);
            state = transition(rand);
            rand = seed.as_mut().next_u32();
        }

        value
    }
}

impl<T: Eq + core::fmt::Debug> Automaton1<T> {
    // TODO might rename to FuzzEngine/FuzzAutomaton
    // Returns the initial state for the automaton
    fn init_state(&self) -> &AutomatonNode1<T> {
        self.initial_node
    }

    fn init_value(&self, seed: u32) -> T {
        (self.generator)(seed)
    }

    // TODO delete this method
    fn seed(&self) -> Box<dyn RngCore> {
        Box::new(thread_rng())
    }

    fn generate(&self) -> T {
        let mut seed: Box<dyn RngCore> = self.seed();

        self.traverse(self.init_value(seed.as_mut().next_u32()))
    }

    // Traverses the graph and computes the end value
    fn traverse(&self, input: T) -> T {
        // !TODO rename to mutate ?
        let mut seed: Box<dyn RngCore> = self.seed();
        let mut value: T = input;
        let mut state: Option<&AutomatonNode1<T>> = Some(self.init_state());
        let mut rand = seed.as_mut().next_u32();
        while let Some(AutomatonNode1 {
            transition,
            transformation,
        }) = state
        {
            value = transformation(value);
            state = transition(rand);
            rand = seed.as_mut().next_u32();
        }

        value
    }
}
