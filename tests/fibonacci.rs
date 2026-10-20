use numkit::{fibonacci, fibonacci_big, BigUint, NumError, MAX_FIBONACCI_INPUT};

#[test]
fn fibonacci_of_0_and_1() {
    assert_eq!(fibonacci(0), Ok(0));
    assert_eq!(fibonacci(1), Ok(1));
}

#[test]
fn the_first_fibonacci_numbers() {
    let expected = [0u64, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    for (n, want) in expected.iter().enumerate() {
        assert_eq!(fibonacci(n as i64), Ok(*want), "fibonacci({n})");
    }
}

#[test]
fn fibonacci_of_the_largest_accepted_input() {
    assert_eq!(MAX_FIBONACCI_INPUT, 93);
    assert_eq!(fibonacci(93), Ok(12200160415121876738));
}

#[test]
fn fibonacci_rejects_out_of_range_integers() {
    for n in [-1, 94] {
        assert!(matches!(fibonacci(n), Err(NumError::OutOfRange(_))), "fibonacci({n})");
    }
}

#[test]
fn fibonacci_big_agrees_with_fibonacci_up_to_the_limit() {
    for n in 0..=MAX_FIBONACCI_INPUT {
        assert_eq!(fibonacci_big(n), Ok(BigUint::from(fibonacci(n).unwrap())), "fibonacci_big({n})");
    }
}

#[test]
fn fibonacci_big_goes_beyond_the_limit() {
    assert_eq!(fibonacci_big(94).unwrap().to_string(), "19740274219868223167");
    assert_eq!(fibonacci_big(100).unwrap().to_string(), "354224848179261915075");
}

#[test]
fn fibonacci_big_validates_input_with_no_upper_bound() {
    assert!(matches!(fibonacci_big(-1), Err(NumError::OutOfRange(_))));
    assert!(fibonacci_big(1000).is_ok());
}
