use crate::big_uint::BigUint;
use crate::error::NumError;

/// The longest sequence fibonacci_sequence builds (specs/004-fibonacci-sequence).
pub const MAX_SEQUENCE_LENGTH: i64 = 10000;

/// The largest n whose Fibonacci number fits in a u64.
pub const MAX_FIBONACCI_INPUT: i64 = 93;

fn check_non_negative(name: &str, n: i64) -> Result<(), NumError> {
    if n < 0 {
        return Err(NumError::OutOfRange(format!("{name} expects n >= 0, got {n}")));
    }
    Ok(())
}

/// F(n) for an integer n from 0 to 93 (specs/002-fibonacci, 003), computed iteratively in O(n).
pub fn fibonacci(n: i64) -> Result<u64, NumError> {
    check_non_negative("fibonacci", n)?;
    if n > MAX_FIBONACCI_INPUT {
        return Err(NumError::OutOfRange(format!(
            "fibonacci expects n <= {MAX_FIBONACCI_INPUT}, got {n}; use fibonacci_big"
        )));
    }
    if n == 0 {
        return Ok(0);
    }
    let (mut previous, mut current) = (0u64, 1u64);
    for _ in 1..n {
        (previous, current) = (current, previous + current);
    }
    Ok(current)
}

/// F(n) for any integer n >= 0 (specs/003-fibonacci-validation-bigint), iterative, O(n).
pub fn fibonacci_big(n: i64) -> Result<BigUint, NumError> {
    check_non_negative("fibonacci_big", n)?;
    if n == 0 {
        return Ok(BigUint::zero());
    }
    let (mut previous, mut current) = (BigUint::zero(), BigUint::from(1));
    for _ in 1..n {
        let next = &previous + &current;
        previous = current;
        current = next;
    }
    Ok(current)
}

/// [F(0), ..., F(count - 1)], built in one pass (specs/004-fibonacci-sequence).
/// Every call builds and returns a new vector.
pub fn fibonacci_sequence(count: i64) -> Result<Vec<BigUint>, NumError> {
    check_non_negative("fibonacci_sequence", count)?;
    if count > MAX_SEQUENCE_LENGTH {
        return Err(NumError::OutOfRange(format!(
            "fibonacci_sequence expects count <= {MAX_SEQUENCE_LENGTH}, got {count}"
        )));
    }
    let mut sequence = Vec::with_capacity(count as usize);
    let (mut previous, mut current) = (BigUint::zero(), BigUint::from(1));
    for _ in 0..count {
        sequence.push(previous.clone());
        let next = &previous + &current;
        previous = current;
        current = next;
    }
    Ok(sequence)
}
