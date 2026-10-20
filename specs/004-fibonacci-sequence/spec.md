# 004 — fibonacci sequence

## Goal

Provide `fibonacci_sequence(count)`, the first `count` Fibonacci numbers, for callers that need
the whole sequence instead of a single term.

## Requirements

1. `fibonacci_sequence(count: i64) -> Result<Vec<BigUint>, NumError>` is exported from
   `src/fibonacci.rs` and returns `count` values: `[F(0), F(1), ..., F(count - 1)]`.
   `fibonacci_sequence(0)` is an empty vector.
2. `count` is validated like `fibonacci_big(n)`: `Err(NumError::OutOfRange(_))` when it is below
   0. It is also capped: the same error when it is above 10000.
3. The sequence is built in a single pass, O(count). Calling `fibonacci_big` once per term
   (O(count²)) does not meet this requirement.
4. Every call returns a new vector. A caller may sort, modify or truncate what it received, and no
   later call returns a different result because of it. In particular, a static cache, if one is
   used, is never handed to the caller.
5. `fibonacci_sequence(10000)` completes in under 100 ms on a current laptop.
6. `README.md` lists this spec and shows `fibonacci_sequence` in the usage example.
