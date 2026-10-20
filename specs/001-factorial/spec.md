# 001 — factorial

## Goal

Provide `factorial(n)`, the product of all positive integers up to `n`.

## Requirements

1. `factorial(n: i64) -> Result<u64, NumError>` is exported from `src/factorial.rs` and
   re-exported by the crate root.
2. `factorial(0)` is `Ok(1)`.
3. For an integer `n` from 1 to 20, `factorial(n)` returns `Ok(n!)`. 20! is the largest factorial
   that fits in a `u64`.
4. Any other input returns `Err(NumError::OutOfRange(_))`: `n` below 0 or above 20.

## Out of scope

Factorials above 20 (they need a big integer type).
