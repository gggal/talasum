use super::Automaton;
use super::LexerRule;

#[derive(Parser)]
//#[derive(Tokenizer)] // add a macro function that generates an alias function for parse
#[grammar = "../resources/yaml.pest"]
pub struct YamlLexer;

impl LexerRule for Rule {
    fn pest_to_automaton(self) -> Option<&'static Automaton<String>> {
        match &self {
            // Rule::string => Some(&super::STRING_AUTOMATON),
            // Rule::number => Some(&super::NUMBER_AUTOMATON),
            // Rule::boolean => Some(&super::BOOL_AUTOMATON),
            // Rule::object => Some(&super::OBJECT_AUTOMATON),
            // Rule::array => Some(&super::ARRAY_AUTOMATON),
            // Rule::null => Some(&super::NULL_AUTOMATON),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::AutomatonToken;

    use super::{YamlLexer, Rule};

    fn tokenize_yaml_input_helper(input: &str) -> Vec<AutomatonToken> {
        super::super::tokenize_input::<YamlLexer, Rule>(input, Rule::value).unwrap()
    }
    #[test]
    fn tokenize_scalar() {
        // valid yaml input shouldn't cause panic
        tokenize_yaml_input_helper("a");
    }

    // #[test]
    // fn tokenize_true_boolean() {
    //     let result = tokenize_yaml_input_helper("true");
    //     assert_eq!(result.len(), 1);

    //     assert_eq!(result[0].from, 0);
    //     assert_eq!(result[0].to, 4);
    // }

    // #[test]
    // fn tokenize_false_boolean() {
    //     let result = tokenize_yaml_input_helper("false");
    //     assert_eq!(result.len(), 1);

    //     assert_eq!(result[0].from, 0);
    //     assert_eq!(result[0].to, 5);
    // }

    // #[test]
    // fn tokenize_string() {
    //     let result = tokenize_yaml_input_helper("\"asd\"");
    //     assert_eq!(result.len(), 1);

    //     assert_eq!(result[0].from, 0);
    //     assert_eq!(result[0].to, 5);
    // }

    // #[test]
    // fn tokenize_object() {
    //     let result = tokenize_yaml_input_helper("{\"a\":1}");
    //     assert_eq!(result.len(), 3);

    //     assert_eq!(result[1].from, 1);
    //     assert_eq!(result[1].to, 4);
    //     assert_eq!(result[0].from, 5);
    //     assert_eq!(result[0].to, 6);
    //     assert_eq!(result[2].from, 0);
    //     assert_eq!(result[2].to, 7);
    // }

    // #[test]
    // fn tokenize_array() {
    //     let result = tokenize_yaml_input_helper("[1,2,3]");
    //     assert_eq!(result.len(), 4);

    //     assert_eq!(result[2].from, 1);
    //     assert_eq!(result[2].to, 2);
    //     assert_eq!(result[1].from, 3);
    //     assert_eq!(result[1].to, 4);
    //     assert_eq!(result[0].from, 5);
    //     assert_eq!(result[0].to, 6);
    //     assert_eq!(result[3].from, 0);
    //     assert_eq!(result[3].to, 7);
    // }

    // #[test]
    // #[should_panic]
    // fn fail_to_tokenize_invalid_yaml() {
    //     tokenize_yaml_input_helper("asd");
    // }
}
