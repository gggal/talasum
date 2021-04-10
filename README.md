Magi is a fuzzing library for generational and modification fuzzing. It provides support for generic types and structures, as well as the following protocols/DSL:
    - JSON
    - YAML
    - HTTP

Magi's aim is to be a zero-abstraction universal fuzzer that can be used for any kind of protocol - binary, network, etc, as long as good insight on edge cases is provided.

Performance

...

Usage

1. Add to Cargo.toml:
   ```[dependencies]
      magi = "0.11"
    ```
2. 