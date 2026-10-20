use numkit::{fibonacci, MAX_FIBONACCI_INPUT};

#[test]
fn fibonacci_of_0_and_1() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
}

#[test]
fn the_first_fibonacci_numbers() {
    let expected = [0u64, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    for (n, want) in expected.iter().enumerate() {
        assert_eq!(fibonacci(n as u32), *want, "fibonacci({n})");
    }
}

#[test]
fn fibonacci_of_the_largest_accepted_input() {
    assert_eq!(MAX_FIBONACCI_INPUT, 93);
    assert_eq!(fibonacci(93), 12200160415121876738);
}
