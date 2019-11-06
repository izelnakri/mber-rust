- REPL
- Default function arguments
- HashMap literal macros (hashmap!{}, btreemap!{} etc) -> maplit create should be part of rust
- test reporters
- test.serial?
- test afterEach
- npm scripts functionality(dynamically retrieve version and write it to code during compile time)_
- Rust hashmaps are too basic, doesnt have assign and literal method that makes high-level code very hard to write sometimes!
Write something like this in rust:
```rust
Object.assign(EXAMPLE_ENV, {
  APP: Object.assign(EXAMPLE_ENV.APP, {
    autoboot: false,
    name: EXAMPLE_ENV.modulePrefix,
    version: "0.0.0+b5f80b0d"
  })
})
```
- cargo test needs better filtering for unit tests, example: cargo test src/utils/recursive_file_lookup.rs
- better test diffs
- only very primitive types allowed as const during compile type (json const not allowed for example)
- recursive copy in standard library


CHECK:
