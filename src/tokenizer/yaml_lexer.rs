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

    #[test]
    fn tokenizing_sequences_does_not_panic() {
        // flow sequences
        tokenize_yaml_input_helper("[1, 2, 3]");
        tokenize_yaml_input_helper("[[], [], [[], []]]");

        // block sequences
        tokenize_yaml_input_helper("- a\n- b");
        tokenize_yaml_input_helper("- a\n- - b");
        tokenize_yaml_input_helper("- |\n a\n- b");
        tokenize_yaml_input_helper(" - a\n -b");
    }

    #[test]
    fn tokenize_scalar_block_as_sequence_element_correctly() {
        parses_to! {
            parser: YamlLexer,
            input: "- |\n  a\n  b",
            rule: Rule::block_sequence,
            tokens: [
                block_sequence(0, 11, [
                block_scalar(2, 11, [
                    scalar_header(3, 3, []),
                    literal_content(4, 11, [])
                    ])
                ])
            ]
        };
    }

    #[test]
    fn tokenizing_mappings_does_not_panic() {
        // flow mappings
        tokenize_yaml_input_helper("{a: b}");
        tokenize_yaml_input_helper("{:a, b:}");
        tokenize_yaml_input_helper("{a, :b, }");

        // block mappings
        tokenize_yaml_input_helper("a: b\nb: c");
        tokenize_yaml_input_helper(" a: b\n b: c");
    }

    #[test]
    fn tokenize_scalar_block_as_mapping_element_correctly() {
        parses_to! {
            parser: YamlLexer,
            input: "a: |\n  a\n  b",
            rule: Rule::block_mapping,
            tokens: [
                block_mapping(0, 12, [
                block_scalar(3, 12, [
                    scalar_header(4, 4, []),
                    literal_content(5, 12, [])
                    ])
                ])
            ]
        };
    }

}
