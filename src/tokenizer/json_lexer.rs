use super::Automaton;
use super::LexerRule;

#[derive(Parser)]
//#[derive(Tokenizer)] // add a macro function that generates an alias function for parse
#[grammar = "../resources/json.pest"]
pub struct JsonLexer;

impl LexerRule for Rule {
    fn pest_to_automaton(self) -> Option<&'static Automaton<String>> {
        match &self {
            Rule::string => Some(&super::NULL_AUTOMATON),
            Rule::number => Some(&super::NUMBER_AUTOMATON),
            Rule::boolean => Some(&super::BOOL_AUTOMATON),
            Rule::object => Some(&super::BOOL_AUTOMATON),
            Rule::array => Some(&super::BOOL_AUTOMATON),
            Rule::null => Some(&super::NULL_AUTOMATON),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::AutomatonToken;

    use super::{JsonLexer, Rule};

    fn tokenize_json_input_helper(input: &str) -> Vec<AutomatonToken> {
        super::super::tokenize_input::<JsonLexer, Rule>(input, Rule::value)
    }
    #[test]
    fn tokenize_null() {
        let result = tokenize_json_input_helper("null");
        assert_eq!(result.len(), 1);

        assert_eq!(result[0].from, 0);
        assert_eq!(result[0].to, 4);
    }

    #[test]
    fn tokenize_true_boolean() {
        let result = tokenize_json_input_helper("true");
        assert_eq!(result.len(), 1);

        assert_eq!(result[0].from, 0);
        assert_eq!(result[0].to, 4);
    }

    #[test]
    fn tokenize_false_boolean() {
        let result = tokenize_json_input_helper("false");
        assert_eq!(result.len(), 1);

        assert_eq!(result[0].from, 0);
        assert_eq!(result[0].to, 5);
    }

    #[test]
    fn tokenize_string() {
        let result = tokenize_json_input_helper("\"asd\"");
        assert_eq!(result.len(), 1);

        assert_eq!(result[0].from, 0);
        assert_eq!(result[0].to, 5);
    }

    #[test]
    fn tokenize_object() {
        let result = tokenize_json_input_helper("{\"a\":1}");
        assert_eq!(result.len(), 3);

        assert_eq!(result[1].from, 1);
        assert_eq!(result[1].to, 4);
        assert_eq!(result[0].from, 5);
        assert_eq!(result[0].to, 6);
        assert_eq!(result[2].from, 0);
        assert_eq!(result[2].to, 7);
    }

    #[test]
    fn tokenize_array() {
        let result = tokenize_json_input_helper("[1,2,3]");
        assert_eq!(result.len(), 4);

        assert_eq!(result[2].from, 1);
        assert_eq!(result[2].to, 2);
        assert_eq!(result[1].from, 3);
        assert_eq!(result[1].to, 4);
        assert_eq!(result[0].from, 5);
        assert_eq!(result[0].to, 6);
        assert_eq!(result[3].from, 0);
        assert_eq!(result[3].to, 7);
    }

    #[test]
    #[should_panic(expected = "Can't parse pest tree")]
    fn fail_to_tokenize_invalid_json() {
        let result = tokenize_json_input_helper("asd");
    }

    // fn type_of<T>(_: &T) -> String {
    //     format!("{}", std::any::type_name::<T>())
    // }

    // #[test]
    // // #[should_panic(expected = "Invalid seed")]
    // fn test() {
    //     let text = String::from("[[0, 1],2,3, {\"a\" : 4}]");
    //     let pairs = JsonLexer::parse(Rule::value, &text).unwrap_or_else(|e| panic!("{}", e));
    //     println!("pairs {}", pairs.as_str());
    //     for pair in pairs.flatten() {
    //         println!("my pair {} , type: {}", pair.as_str(), type_of(&pair));
    //         println!(
    //             "my rule {:?}, type: {}",
    //             pair.as_rule(),
    //             type_of(&pair.as_rule())
    //         );
    //         // for token in pair.tokens() {
    //         //     println!("my token {:?}", token);
    //         // }

    //         // let inners = pair.into_inner();
    //         // for inner in inners {
    //         //     println!("my inner {}, type: {}", inner, type_of(&inner));
    //         // }
    //     }
    // }

    // #[test]
    // // #[should_panic(expected = "Invalid seed")]
    // fn test1() {
    //     all_automaton_tokens(&String::from(
    //         "{\"key\":\"SEA-739\",\"status\":{\"Open\": [1,2,3]},\"assignee\":null}",
    //     ));
    // }
}
