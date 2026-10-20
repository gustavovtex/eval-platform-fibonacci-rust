# eval-platform-fibonacci-rust

Small number utilities in plain Rust, with no dependencies.

This repository is a fixture for the [eval platform](https://github.com/vtex/eval-platform): its
pull requests are small, each one carries its own spec under `specs/`, and the whole test suite
runs in under a second. That makes it a cheap way to check that an evaluation works end to end,
locally and in production. It is not meant to tell models apart: the tasks are too easy for that.

Twin repositories have the same history in other languages: `eval-platform-fibonacci-node`, `eval-platform-fibonacci-python`, `eval-platform-fibonacci-go`, `eval-platform-fibonacci-csharp`.

## Usage

```rust
use numkit::{factorial, fibonacci};

assert_eq!(factorial(5), Ok(120));
assert_eq!(fibonacci(10), 55);
```

## Tests

```sh
cargo test
```

Rust 1.75 or newer. No dependencies: the tests use the built-in `cargo test` harness.

## Specs

Every change starts with a spec in `specs/NNN-name/spec.md`.

| Spec | Feature |
|---|---|
| [001-factorial](specs/001-factorial/spec.md) | `factorial(n)` |
| [002-fibonacci](specs/002-fibonacci/spec.md) | `fibonacci(n)` |
