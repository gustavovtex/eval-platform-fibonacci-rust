/// The largest n whose Fibonacci number fits in a u64.
pub const MAX_FIBONACCI_INPUT: u32 = 93;

/// F(n) for an integer n from 0 to 93 (specs/002-fibonacci), computed iteratively in O(n).
pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut previous, mut current) = (0u64, 1u64);
    for _ in 1..n {
        (previous, current) = (current, previous + current);
    }
    current
}
