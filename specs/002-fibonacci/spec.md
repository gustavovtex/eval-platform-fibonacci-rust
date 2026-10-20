# 002 — fibonacci

## Goal

Provide `fibonacci(n)`, the n-th Fibonacci number, with `F(0) = 0`, `F(1) = 1` and
`F(n) = F(n - 1) + F(n - 2)`.

## Requirements

1. `fibonacci(n: u32) -> u64` is exported from `src/fibonacci.rs` and re-exported by the crate
   root.
2. For an integer `n` from 0 to 93, `fibonacci(n)` returns `F(n)`. F(93) is the largest Fibonacci
   number that fits in a `u64`.
3. The computation is iterative and runs in O(n) time: no recursion, no exponential blow-up.
4. `README.md` lists the new function in its specs table and shows a usage example.

## Out of scope

Input validation and values above F(93). They come in a later spec.
