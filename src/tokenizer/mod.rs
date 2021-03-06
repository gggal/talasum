pub mod json_lexer;
pub mod yaml_lexer;

use super::state_machine::Automaton;

/// This module is used for PEG-parsable (e.g. text) protocols.
/// It expects that a state_machine logic and a FSM-to-PEG mapping
/// has been defined. For binary protocols go to [?].`
use pest::{Parser, RuleType};

/// This trait is mandatory for Rule enums from all Pest implementations
pub trait LexerRule: RuleType {
    /// maps the underlying rule to its inner representation as Automaton
    fn pest_to_automaton(self) -> Option<&'static Automaton<String>>;
}

/// Representation of a single token - characterized by:
/// - its first position in the input text
/// - its last position in the input text
/// - its corresponding automaton
pub struct AutomatonToken<'a> {
    pub from: usize,
    pub to: usize,
    pub automaton: &'a Automaton<String>,
}

/// Converts a Pest pair to its corresponding token
fn pest_pair_to_token<'a, T: 'a + LexerRule>(
    pair: &pest::iterators::Pair<T>,
) -> Option<AutomatonToken<'a>> {
    let rule = pair.as_rule();
    let start = pair.as_span().start();
    let end = pair.as_span().end();

    rule.pest_to_automaton().map(|rule| AutomatonToken {
        from: start,
        to: end,
        automaton: rule,
    })
}

/// Produces a list of (u32, u32, String) element, each representing
/// a separate token, as defined by the state_machine module
pub fn tokenize_input<'a, P: Parser<R>, R: 'a + LexerRule>(
    text: &'a str,
    parent_rule: R,
) -> Option<Vec<AutomatonToken>> {
    if text.is_empty() {
        Some(vec![])
    } else {
        match P::parse(parent_rule, text) {
            Ok(pairs) => Some(tokenize_peg_tree::<R>(pairs)),
            Err(_) => None,
        }
    }
}

/// Iterates through all pairs in a Pest tree and generates a list of tokens
/// in an order such that each element doesn't depend on another after it
fn tokenize_peg_tree<'a, T: 'a + LexerRule>(
    tree_root: pest::iterators::Pairs<'a, T>,
) -> Vec<AutomatonToken> {
    tree_root
        .flatten()
        .filter_map(|aut| pest_pair_to_token::<T>(&aut))
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {

    #[derive(Parser)]
    #[grammar = "../resources/mock.pest"]
    pub struct MockLexer;
    use crate::tokenizer::AutomatonToken;

    use crate::state_machine::json::boolean::BOOL_AUTOMATON;
    use crate::state_machine::json::null::NULL_AUTOMATON;

    use super::Automaton;
    use pest::Parser;

    impl super::LexerRule for Rule {
        fn pest_to_automaton(self) -> Option<&'static Automaton<String>> {
            match &self {
                Rule::inner => Some(&BOOL_AUTOMATON),
                Rule::nested => Some(&NULL_AUTOMATON),
                _ => None, // not every Pest token will have Automaton representation
            }
        }
    }

    #[test]
    fn try_tokenizing_text_with_wrong_parent_rule() {
        assert!(
            super::tokenize_input::<MockLexer, Rule>("(1)", Rule::not_a_nested_token).is_none()
        );
    }

    #[test]
    fn tokenize_input_successfully() {
        let result = super::tokenize_input::<MockLexer, Rule>("(((1)))", Rule::nested).unwrap();
        assert_eq!(result.len(), 4);

        assert_eq!(result[0].from, 3);
        assert_eq!(result[0].to, 4);

        assert_eq!(result[1].from, 2);
        assert_eq!(result[1].to, 5);

        assert_eq!(result[2].from, 1);
        assert_eq!(result[2].to, 6);

        assert_eq!(result[3].from, 0);
        assert_eq!(result[3].to, 7);
    }

    fn tokenize_peg_tree_helper(rule: Rule, text: &str) -> Vec<AutomatonToken> {
        let pairs =
            MockLexer::parse(rule, text).unwrap_or_else(|e| panic!("Invalid test setup: {}", e));
        super::tokenize_peg_tree(pairs)
    }

    #[test]
    fn iterate_pest_with_single_top_element_not_recognized() {
        let result = tokenize_peg_tree_helper(Rule::not_a_token, "0");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn iterate_pest_with_single_top_element_recognized() {
        let result = tokenize_peg_tree_helper(Rule::inner, "1");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].from, 0);
        assert_eq!(result[0].to, 1);
    }

    #[test]
    fn iterate_pest_with_all_elements_recognized() {
        let result = tokenize_peg_tree_helper(Rule::nested, "(1)");
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].from, 1);
        assert_eq!(result[0].to, 2);

        assert_eq!(result[1].from, 0);
        assert_eq!(result[1].to, 3);
    }

    #[test]
    fn iterate_pest_with_all_elements_not_recognized() {
        let result = tokenize_peg_tree_helper(Rule::not_a_nested_token, "[0]");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn iterate_pest_while_preserving_order() {
        let result = tokenize_peg_tree_helper(Rule::nested, "(((1)))");
        assert_eq!(result.len(), 4);

        assert_eq!(result[0].from, 3);
        assert_eq!(result[0].to, 4);

        assert_eq!(result[1].from, 2);
        assert_eq!(result[1].to, 5);

        assert_eq!(result[2].from, 1);
        assert_eq!(result[2].to, 6);

        assert_eq!(result[3].from, 0);
        assert_eq!(result[3].to, 7);
    }

    #[test]
    fn iterate_pest_with_only_top_element_not_recognized() {
        let result = tokenize_peg_tree_helper(Rule::not_a_nested_token, "[1]");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].from, 1);
        assert_eq!(result[0].to, 2);
    }

    #[test]
    fn iterate_pest_with_only_top_element_recognized() {
        let result = tokenize_peg_tree_helper(Rule::nested, "(0)");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].from, 0);
        assert_eq!(result[0].to, 3);
    }
}
