use super::Automaton;
use super::LexerRule;

#[derive(Parser)]
//#[derive(Tokenizer)] // add a macro function that generates an alias function for parse
#[grammar = "../resources/yaml.pest"]
pub struct YamlLexer;

impl LexerRule for Rule {
    fn pest_to_automaton(self) -> Option<&'static Automaton<String>> {
        match &self {
            Rule::scalar => Some(&super::BOOL_AUTOMATON),
            Rule::mapping => Some(&super::STRING_AUTOMATON),
            Rule::sequence => Some(&super::NUMBER_AUTOMATON),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::{consumes_to, parses_to};

    use crate::tokenizer::AutomatonToken;

    use super::{Rule, YamlLexer};

    fn tokenize_yaml_input_helper(input: &str) -> Vec<AutomatonToken> {
        super::super::tokenize_input::<YamlLexer, Rule>(input, Rule::value).unwrap()
    }
    #[test]
    fn tokenizing_scalars_does_not_panic() {
        // flow scalars
        tokenize_yaml_input_helper("a");
        tokenize_yaml_input_helper("'a'");
        tokenize_yaml_input_helper("\"a\"");

        // block scalars
        tokenize_yaml_input_helper("|\n a");
        tokenize_yaml_input_helper("|\n  a");
        tokenize_yaml_input_helper("|\n   a");
        tokenize_yaml_input_helper("|+\n a");
        tokenize_yaml_input_helper("|-\n a");
        tokenize_yaml_input_helper("|1\n a #comment");
        tokenize_yaml_input_helper(">\n a");
        tokenize_yaml_input_helper(">2-\n a");
        tokenize_yaml_input_helper("  |\n  a\n  b");
    }

    #[test]
    #[should_panic]
    fn tokenizing_ill_indented_block_scalars_panics() {
        parses_to! {
            parser: YamlLexer,
            input: " |\n a",
            rule: Rule::block_scalar,
            tokens: [
                block_scalar(0, 5, [])
            ]
        };
    }

    #[test]
    fn tokenize_multiline_block_scalar_correctly() {
        parses_to! {
            parser: YamlLexer,
            input: "|\n a\n b\n  c",
            rule: Rule::block_scalar,
            tokens: [
                block_scalar(0, 11, [
                    scalar_header(1, 1, []),
                    literal_content(2, 11, [])
                    ])
            ]
        };
    }

    #[test]
    fn tokenize_comments_in_scalar_correctly() {
        parses_to! {
            parser: YamlLexer,
            input: "| #comment\n a",
            rule: Rule::block_scalar,
            tokens: [
                block_scalar(0, 13, [
                    scalar_header(1, 1, []),
                    literal_content(11, 13, [])
                    ])
            ]
        };
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
