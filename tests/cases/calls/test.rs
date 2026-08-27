
extern "Rust" {
    fn fib(n: u64) -> u64;
}

fn main() {
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 2);
    assert_eq!(fib(3), 3);
    assert_eq!(fib(4), 5);
    assert_eq!(fib(5), 8);
    assert_eq!(fib(6), 13);
    assert_eq!(fib(7), 21);
    assert_eq!(fib(8), 34);
    assert_eq!(fib(9), 55);
    assert_eq!(fib(10), 89);
    assert_eq!(fib(11), 144);
}