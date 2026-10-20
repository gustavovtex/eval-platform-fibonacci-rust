use crate::error::NumError;

/// The largest n whose factorial fits in a u64.
pub const MAX_FACTORIAL_INPUT: i64 = 20;

/// n! for an integer n from 0 to 20 (specs/001-factorial).
pub fn factorial(n: i64) -> Result<u64, NumError> {
    if !(0..=MAX_FACTORIAL_INPUT).contains(&n) {
        return Err(NumError::OutOfRange(format!(
            "factorial expects 0 <= n <= {MAX_FACTORIAL_INPUT}, got {n}"
        )));
    }
    let mut result: u64 = 1;
    for i in 2..=n as u64 {
        result *= i;
    }
    Ok(result)
}
