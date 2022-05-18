use super::Automaton;
use super::LexerRule;

use crate::state_machine::yaml::flow_scalar::FLOW_SCALAR_AUTOMATON;
use crate::state_machine::yaml::indentation::INDENTATION_AUTOMATON;

#[derive(Parser)]
//#[derive(Tokenizer)] // add a macro function that generates an alias function for parse
#[grammar = "../resources/yaml.pest"]
pub struct YamlLexer;

impl LexerRule for Rule {
    fn pest_to_automaton(self) -> Option<&'static Automaton<String>> {
        match &self {
            Rule::indent => Some(&INDENTATION_AUTOMATON),
            Rule::indent_nonempty => Some(&INDENTATION_AUTOMATON),
            Rule::spaces => Some(&INDENTATION_AUTOMATON),
            Rule::flow_scalar => Some(&FLOW_SCALAR_AUTOMATON),
            // Rule::mapping => Some(&super::STRING_AUTOMATON),
            // Rule::sequence => Some(&super::NUMBER_AUTOMATON),
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
    #[should_panic]
    fn tokenizing_unescaped_chars_in_double_quoted_literal_panics() {
        parses_to! {
            parser: YamlLexer,
            input: "\"a\"b\"",
            rule: Rule::scalar,
            tokens: [
                block_scalar(0, 5, [])
            ]
        };
    }

    #[test]
    #[should_panic]
    fn tokenizing_unescaped_chars_in_single_quoted_literal_panics() {
        parses_to! {
            parser: YamlLexer,
            input: "'a'b'",
            rule: Rule::scalar,
            tokens: [
                block_scalar(0, 5, [])
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
        tokenize_yaml_input_helper(" - a\n - b");
    }

    #[test]
    fn tokenizing_mappings_does_not_panic() {
        // flow mappings
        tokenize_yaml_input_helper("{a: b}");
        tokenize_yaml_input_helper("{:a, b:}");
        tokenize_yaml_input_helper("{a, :b, }");

        // block mappings
        tokenize_yaml_input_helper("a: b\nb: c");
        // tokenize_yaml_input_helper(" a: b\n b: c");
    }

    #[test]
    fn tokenizing_documents_does_not_panic() {
        tokenize_yaml_input_helper("---");
        tokenize_yaml_input_helper("...");
        tokenize_yaml_input_helper(
            "---
        # some comment",
        );
        tokenize_yaml_input_helper(
            "---
        # some comment
        ",
        );
    }

    #[test]
    fn tokenizing_tags_does_not_panic() {
        tokenize_yaml_input_helper("!!str asd");
        tokenize_yaml_input_helper("!!map &a2 baz : *a1");
        tokenize_yaml_input_helper("!!str baz : !!str *a1");
        tokenize_yaml_input_helper("asd : !!str as");
        tokenize_yaml_input_helper(
            "sequence: !!seq
        - entry",
        );
        tokenize_yaml_input_helper(
            "!!str |-
        'String: just a theory.'",
        );
    }

    #[test]
    fn tokenizing_anchors_does_not_panic() {
        tokenize_yaml_input_helper("---");
    }

    // some examples from the YAML specification

    #[test]
    fn example_2_3_works() {
        tokenize_yaml_input_helper(
            r"american:
- Boston Red Sox
- Detroit Tigers
- New York Yankees
national:
- New York Mets
- Chicago Cubs
- Atlanta Braves",
        );
    }

    #[test]
    fn example_2_4_works() {
        tokenize_yaml_input_helper(
            r"-
 name: Mark McGwire
 hr:   65
 avg:  0.278
-
 name: Sammy Sosa
 hr:   63
 avg:  0.288",
        );
    }

    //     #[test]
    //     fn example_2_11_works() {
    //         tokenize_yaml_input_helper(
    //             r"? - Detroit Tigers
    //   - Chicago cubs
    // : - 2001-07-23

    // ? [ New York Yankees,
    //     Atlanta Braves ]
    // : [ 2001-07-02, 2001-08-12,
    //     2001-08-14 ]",
    //         );
    //     }

    //     #[test]
    //     fn nested_mapping() {
    //         tokenize_yaml_input_helper(
    //             r"? ? a
    //   : v
    // : a",
    //         );
    //     }

    #[test]
    fn example_7_20_works() {
        tokenize_yaml_input_helper(
            r"[
        ? foo
         bar : baz
        ]",
        );
    }

    // #[test]
    // fn example_7_21_works() {
    //     tokenize_yaml_input_helper(r"- [ YAML : separate ]
    //     - [ : empty key entry ]
    //     - [ {JSON: like}:adjacent ]");
    // }

    #[test]
    #[should_panic]
    fn example_7_22_works() {
        tokenize_yaml_input_helper(
            r#"[ foo
        bar: invalid,
        "foo_...>1K characters..._bar": invalid ]"#,
        );
    }

    #[test]
    fn example_7_23_works() {
        tokenize_yaml_input_helper(
            "- [ a, b ]
- { a: b }
- \"a\"
- 'b'
- c",
        );
    }

    #[test]
    fn example_8_1_works() {
        tokenize_yaml_input_helper(
            "- | # Empty header
 literal
- >1 # Indentation indicator
  folded
- |+ # Chomping indicator
 keep
 
- >1- # Both indicators
  strip",
        );
    }
}
