use numkit::{fibonacci_big, fibonacci_sequence, BigUint, NumError, MAX_SEQUENCE_LENGTH};

#[test]
fn fibonacci_sequence_of_0_is_empty() {
    assert_eq!(fibonacci_sequence(0), Ok(vec![]));
}

#[test]
fn fibonacci_sequence_starts_with_the_first_fibonacci_numbers() {
    let expected: Vec<BigUint> = [0u64, 1, 1, 2, 3, 5, 8, 13].iter().map(|v| BigUint::from(*v)).collect();
    assert_eq!(fibonacci_sequence(8), Ok(expected));
}

#[test]
fn every_term_matches_fibonacci_big() {
    let sequence = fibonacci_sequence(200).unwrap();
    assert_eq!(sequence.len(), 200);
    for (n, term) in sequence.iter().enumerate() {
        assert_eq!(Ok(term.clone()), fibonacci_big(n as i64), "term {n}");
    }
}

#[test]
fn fibonacci_sequence_accepts_the_cap_and_rejects_beyond_it() {
    assert_eq!(MAX_SEQUENCE_LENGTH, 10000);
    assert_eq!(fibonacci_sequence(10000).unwrap().len(), 10000);
    assert!(matches!(fibonacci_sequence(10001), Err(NumError::OutOfRange(_))));
}

#[test]
fn fibonacci_sequence_validates_count() {
    assert!(matches!(fibonacci_sequence(-1), Err(NumError::OutOfRange(_))));
}

#[test]
fn every_call_returns_a_new_vector() {
    let mut first = fibonacci_sequence(5).unwrap();
    first.reverse();
    first.truncate(2);
    let second = fibonacci_sequence(5).unwrap();
    let expected: Vec<BigUint> = [0u64, 1, 1, 2, 3].iter().map(|v| BigUint::from(*v)).collect();
    assert_eq!(second, expected);
}
