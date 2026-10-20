# 003 — fibonacci: input validation and big integers

## Goal

Make `fibonacci(n)` refuse bad input the same way `factorial(n)` does, and add
`fibonacci_big(n)` for Fibonacci numbers beyond F(93).

## Requirements

1. `fibonacci(n: i64)` now returns `Result<u64, NumError>`. It keeps its behavior for integers
   from 0 to 93, and returns `Err(NumError::OutOfRange(_))` when `n` is below 0 or above 93.
2. `fibonacci_big(n: i64) -> Result<BigUint, NumError>` is exported from `src/fibonacci.rs` and
   returns `F(n)` for any integer `n >= 0`. `BigUint` is a small unsigned big integer in
   `src/big_uint.rs`, with `Display`, `Add` and `From<u64>`, and no dependencies.
3. `fibonacci_big(n)` returns `Err(NumError::OutOfRange(_))` when `n` is below 0. There is no upper
   bound.
4. For every `n` from 0 to 93, `fibonacci_big(n)` equals `BigUint::from(fibonacci(n))`.
5. Both functions stay iterative and O(n).
6. `README.md` lists this spec and shows `fibonacci_big` in the usage example.
