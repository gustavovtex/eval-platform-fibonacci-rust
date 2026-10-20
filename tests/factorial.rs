use numkit::{factorial, NumError, MAX_FACTORIAL_INPUT};

#[test]
fn factorial_of_0_and_1_is_1() {
    assert_eq!(factorial(0), Ok(1));
    assert_eq!(factorial(1), Ok(1));
}

#[test]
fn factorial_of_small_integers() {
    assert_eq!(factorial(5), Ok(120));
    assert_eq!(factorial(10), Ok(3628800));
}

#[test]
fn factorial_of_the_largest_accepted_input() {
    assert_eq!(MAX_FACTORIAL_INPUT, 20);
    assert_eq!(factorial(20), Ok(2432902008176640000));
}

#[test]
fn factorial_rejects_out_of_range_integers() {
    for n in [-1, 21] {
        assert!(matches!(factorial(n), Err(NumError::OutOfRange(_))), "factorial({n})");
    }
}
