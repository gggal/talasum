use crate::randomness::{PRandomizer, Randomizer};
pub mod helper;
pub mod json;
pub mod weights;

// Transformation is an operation to be performed on a value as it's moved
// through the automaton
type Transformation<T> = fn(u64, T) -> T;

/// Transition from one state in the automaton to another one adjacent to it.
/// If the value is None, then the state is final and there isn't a state
/// to transition to.
type Transition<T> = Box<dyn Fn(u64) -> Option<&'static AutomatonNode<T>> + std::marker::Sync>;

/// In case of generation-based fuzzing an initial value is generated before
/// it's being sent for traversing.
/// This describes functions that generate initial values based on a seed.
type Generate<T> = fn(u64) -> T;

/// Represents an automaton state and transitions to its adjacent states. It consists of
/// - a transformation function that is applied to the input when the state is visited
/// - a transition function that returns the next state to be visited (if such exists)
/// based on a user-provided seed.
/// Automata constructed out of `AutomatonNode`s are:
/// - finite - each of them is explicitly specified by the user
/// - deterministic (meaning fuzzing is retraceable)
/// - weighted - weight of a node transition is its likeliness to be chosen out of all
/// adjacent nodes
/// - cyclic
///
/// This struct is designed with the intention of its objects being global and immutable,
/// but only loaded in memory if used. Thus, due to lazy_static restrictions, forming
/// cycles is handled differently than the normal way of defining transitions, see
/// `set_edge`/`set_edges` and `set_cycle`.
pub struct AutomatonNode<T: 'static> {
    transition: Transition<T>,
    transformation: Transformation<T>,
    cycle: usize,
}

impl AutomatonNode<String> {
    /// Constructs a trivial automaton node: one without a
    /// transformation function and adjacent nodes.
    fn new() -> Self {
        Self {
            transition: Box::new(|_| None),
            transformation: helper::IDENTITY,
            cycle: 0,
        }
    }

    /// A builder function that adds a single adjacent node to the current one.
    /// This adjacent node must not be amongst the preceding nodes of the current
    /// node. If it is, this will lead to deadlock if initialized in lazy_static block.
    ///
    /// Calling this function overrides previously set transitions.
    ///
    /// If adjacent nodes are not added, the current state is final.
    fn set_edge(mut self, next: &'static Self) -> Self {
        self.transition = Box::new(move |_| Some(next));
        self.cycle = 0;
        self
    }

    /// A builder function that adds a set of adjacent nodes to the current one,
    /// each with its own probability number derived by the user-defined constant
    /// and the v-coef value. Upon execution, one of these nodes will be chosen
    /// based on the seed value.
    ///
    /// Each adjacent node must not be amongst the preceding nodes of the current
    /// node. If it is, this will lead to deadlock if initialized in lazy_static block.
    ///
    /// Calling this function overrides previously set transitions.
    ///
    /// If adjacent nodes are not added, the current state is final.
    fn set_edges(mut self, edges: Vec<(u32, &'static Self)>) -> Self {
        self.transition = weights::choose(
            edges
                .iter()
                .map(|(num, node)| (*num, Some(*node)))
                .collect(),
        );
        self.cycle = 0;
        self
    }

    /// A builder function that adds a transformation function to the current node.
    ///
    /// If a transformation function is not added, execution will proceed with the
    /// next state without changing the fuzzing value.
    fn set_func(mut self, func: Transformation<String>) -> Self {
        self.transformation = func;
        self
    }

    /// A builder function that adds a previously visited node as a transition to
    /// the current one, forming a cycle. `cycle` is number of transitions from the
    /// current node to the visited node. If set to 1, it means that the next node
    /// will be the same as the previous one, if set to 2, it means the next will be
    /// the one before the previous node and so on.
    /// Calling this function overrides previously set transitions.
    ///
    /// If adjacent nodes are not added, the current state is final.
    fn set_cycle(mut self, cycle: usize) -> Self {
        self.cycle = cycle;
        self.transition = Box::new(|_| None);
        self
    }
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
        let mut node_stack: Vec<Option<&AutomatonNode<T>>> = Vec::new();
        let mut seeder = PRandomizer::new(seed);
        let mut value: T = input;
        let mut state: Option<&AutomatonNode<T>> = Some(self.init_state());
        let mut rand: u64;
        while let Some(AutomatonNode {
            transition,
            transformation,
            cycle,
        }) = state
        {
            rand = seeder.get();
            value = transformation(rand, value);
            node_stack.push(state);

            // if the next node has already been visited
            if *cycle != 0 {
                node_stack.truncate(node_stack.len() - *cycle);
                state = *node_stack.last().expect("Invalid automaton definition!");
            } else {
                state = transition(rand);
            }
        }

        value
    }
}
#[cfg(test)]
mod tests {
    use super::{helper::FINAL, helper::IDENTITY, Automaton, AutomatonNode};

    lazy_static! {
        static ref TEST_NODE1: AutomatonNode<String> =
            AutomatonNode::<String>::new().set_func(|_, _| String::from("Test1"));
        static ref TEST_NODE2: AutomatonNode<String> =
            AutomatonNode::<String>::new().set_func(|_, _| String::from("Test2"));
        static ref FIRST: AutomatonNode<String> = AutomatonNode::<String>::new()
            .set_func(|_, text| format!("1{}", text))
            .set_edges(vec![(1, &SECOND), (1, &THIRD)]);
        static ref SECOND: AutomatonNode<String> = AutomatonNode::<String>::new()
            .set_func(|_, text| format!("2{}", text))
            .set_edge(&THIRD);
        static ref THIRD: AutomatonNode<String> =
            AutomatonNode::<String>::new().set_func(|_, text| format!("3{}", text));
        static ref FIRST_CYCLIC: AutomatonNode<String> = AutomatonNode::<String>::new()
            .set_func(|_, text| format!("4{}", text))
            .set_edge(&SECOND_CYCLIC);
        static ref SECOND_CYCLIC: AutomatonNode<String> = AutomatonNode::<String>::new()
            .set_func(|_, text| format!("5{}", text))
            .set_edges(vec![(2, &THIRD_CYCLIC), (1, &FINAL)]);
        static ref THIRD_CYCLIC: AutomatonNode<String> = AutomatonNode::<String>::new()
            .set_func(|_, text| format!("6{}", text))
            .set_cycle(2);
        static ref TEST_AUTOMATON: Automaton<String> = Automaton::<String> {
            initial_node: &FIRST,
            generator: |seed| { seed.to_string() },
        };
        static ref TEST_CYCLIC_AUTOMATON: Automaton<String> = Automaton::<String> {
            initial_node: &FIRST_CYCLIC,
            generator: |seed| { seed.to_string() },
        };
        static ref INVALID_NODE: AutomatonNode<String> =
            AutomatonNode::<String>::new().set_cycle(1);
    }

    #[test]
    fn new_nodes_are_nulled_out() {
        let empty = AutomatonNode::new();
        assert_eq!(empty.transformation, IDENTITY);
        assert_eq!(empty.cycle, 0);
    }

    #[test]
    fn setting_single_edge_resets_transition() {
        let node = AutomatonNode::new()
            .set_cycle(1)
            .set_edges(vec![(1, &TEST_NODE1), (1, &TEST_NODE2)])
            .set_edge(&TEST_NODE1);
        assert_eq!(
            ((node.transition)(123).unwrap().transformation)(0, String::new()),
            "Test1"
        );
        assert_eq!(node.cycle, 0);
    }

    #[test]
    fn setting_single_edge_correctly() {
        let node = AutomatonNode::new().set_edge(&TEST_NODE1);
        assert_eq!(
            ((node.transition)(123).unwrap().transformation)(0, String::new()),
            "Test1"
        );
    }

    #[test]
    fn setting_multiple_edges_resets_transition() {
        let node = AutomatonNode::new()
            .set_cycle(1)
            .set_edge(&TEST_NODE1)
            .set_edges(vec![(1, &TEST_NODE2)]);
        assert_eq!(
            ((node.transition)(123).unwrap().transformation)(0, String::new()),
            "Test2"
        );
        assert_eq!(node.cycle, 0);
    }

    #[test]
    fn setting_multiple_edges_when_list_is_empty_does_nothing() {
        assert!((AutomatonNode::new().set_edges(vec![]).transition)(123).is_none());
    }

    #[test]
    fn setting_multiple_edges_correctly() {
        let node1 = AutomatonNode::new().set_edges(vec![(1, &TEST_NODE1), (10000, &TEST_NODE2)]);
        let node2 = AutomatonNode::new().set_edges(vec![(10000, &TEST_NODE1), (1, &TEST_NODE2)]);

        assert_eq!(
            ((node1.transition)(123).unwrap().transformation)(0, String::new()),
            "Test2"
        );
        assert_eq!(
            ((node2.transition)(123).unwrap().transformation)(0, String::new()),
            "Test1"
        );
    }

    #[test]
    fn setting_cycle_resets_transition() {
        let node = AutomatonNode::new()
            .set_edge(&TEST_NODE1)
            .set_edges(vec![(1, &TEST_NODE2)])
            .set_cycle(1);
        assert!((node.transition)(123).is_none());
        assert_eq!(node.cycle, 1);
    }

    #[test]
    fn setting_cycle_correctly() {
        assert_eq!(AutomatonNode::new().set_cycle(123).cycle, 123);
    }

    #[test]
    fn setting_transformation_correctly() {
        let node = AutomatonNode::new().set_func(|_, _| String::from("works"));
        assert_eq!(
            (node.transformation)(0, String::new()),
            String::from("works")
        );
    }

    #[test]
    fn init_state_is_set_correctly() {
        assert_eq!(
            (TEST_AUTOMATON.init_state().transformation)(0, String::from("")),
            String::from("1")
        );
    }

    #[test]
    fn init_value_is_generated_correctly() {
        assert_eq!(TEST_AUTOMATON.init_value(123), String::from("123"));
    }

    #[test]
    fn generate_acts_as_traverse_for_init_value() {
        for i in 1..101 {
            assert_eq!(
                TEST_AUTOMATON.generate(i),
                TEST_AUTOMATON.traverse(i.to_string(), i)
            );
        }
    }

    #[test]
    #[should_panic(expected = "Invalid automaton definition!")]
    fn traverse_panics_if_cycle_number_is_invalid() {
        let invalid_automata = Automaton::<String> {
            initial_node: &INVALID_NODE,
            generator: |seed| seed.to_string(),
        };
        invalid_automata.generate(123);
    }

    #[test]
    fn traversal_depends_on_input_for_acyclic_automaton() {
        assert_ne!(
            TEST_AUTOMATON.traverse(String::from("1"), 123),
            TEST_AUTOMATON.traverse(String::from("2"), 123)
        );
    }

    #[test]
    fn fuzzing_works_as_expected_for_acyclic_automaton() {
        assert_eq!(TEST_AUTOMATON.generate(123), String::from("321123"));
    }

    #[test]
    fn traversal_depends_on_input_for_cyclic_automaton() {
        assert_ne!(
            TEST_CYCLIC_AUTOMATON.traverse(String::from("1"), 123),
            TEST_CYCLIC_AUTOMATON.traverse(String::from("2"), 123)
        );
    }

    #[test]
    fn fuzzing_works_as_expected_for_cyclic_automaton() {
        // ends_with is used because we don't want this test to depend on
        // the number of repetitions in the cycle
        assert!(TEST_CYCLIC_AUTOMATON.generate(123).ends_with("654123"));
    }
}
