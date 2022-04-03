## Summary

Magi is a protocol-aware fuzzing library, capable of both mutation and generation fuzzing. Its aim is to be a zero-cost abstraction universal fuzzer that can be used for any kind of protocol - binary or textual.

Regardless fuzzing type and protocol, a seed needs to be provided (alongside the input for mutation fuzzing). Based on it Magi generates sequences of pseudo-random fuzzed values.


## Supported protocols

So far the supported protocols are:
  - JSON
  - YAML

and plans are in place for the following:
  - HTTP
  - Unicode
  - GRPC
  - Markdown

## How to use


1. Add to Cargo.toml:
    ```
    [dependencies]
    magi = "0.1.0"
    ```

2. Generate random input
    ```rust
    use magi::json;
    
    let seed = 1234;
    for fuzzed in json::boolean(seed).take(10) {
        println!("New boolean value: {}", fuzzed);
    }
     ```
    List of types that can be generated can be found [here](todo).

3. Mutate user-provided input
    ```rust
    use magi::json;
    
    let seed = 1234;
    match json::mutate(
        "{\"a\": 123, \"b\": [null, true, \"c\"]}", 
    seed) {
        Some(mutator) => {
            for fuzzed in mutator.take(10) {
                println!("New value: {}", fuzzed);
            }
        },
        None => panic!("Your input was not a valid JSON document")
     }
    ```
    *Mutation can be applied for all supported protocols, you just need to provide valid input as per chosen protocol's specification.

## How to configure

There are two measurements the user can configure in order to control the scale of the fuzzing process:

1. Horizontal fuzzing coefficient

    Horizontal randomness determines how extreme the changes to the user provided input are during mutation. This coefficient is only relevant for mutation of nested tokens, like a JSON array of object. A high h-coef value means more severe changes to user's input, thus max h-coef makes the mutator behave as a generator. A lower h-coef value means more subtle changes during mutation (but at least one token will be mutated at all times).

    Supported values are integers from 1 to 100 (incl), 1 being the min possible value and 100 being the max possible value. **The default value is 50.**

    One can configure it by passing a MAGI_HORIZONTAL_RANDOMNESS_COEF environment variable.

2. Vertical fuzzing coefficient
   
    Vertical randomness determines how extreme the fuzzed result sequence for individual tokens is. The higher this coefficient is, the more frequently edge cases occur in the output of the fuzzer. And, respectively, the lower this value is, the more thorough the fuzzer is in generating new values. It is recommended that this coefficient is set to a higher value when the user doesn't have a lot of tries and needs to exhaust as many edge cases as fast as possible. On the contrary, if the user can afford a longer fuzzing process, this coefficient should be set to a lower value in order to ensure covering as many cases as possible.

    To illustrate the need for this coefficient, let's say we have only 10 tries to generate fuzz values for a JSON string. We want for the value to be null for (at least) one of these tries, as that is a common and important edge case for any JSON type. This amounts to 1/10th of all fuzz values being null. However, if we have 10_000 tries instead, we surely don't want for 1/10th, or one thousand, of all fuzz values to be null as this would be a waste of tries.

    Supported values are integers from 1 to 100 (incl), 1 being the min possible value and 100 being the max possible value. **The default value is 50.**

    One can configure it by passing a MAGI_VERTICAL_RANDOMNESS_COEF environment variable.