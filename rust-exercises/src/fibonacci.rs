#![allow(dead_code)]
use std::thread;

pub fn fibonacci_recursive(n: &u128) -> u128 {
    if *n <= 1 {
        *n
    } else {
        fibonacci_recursive(&(*n - 1)) + fibonacci_recursive(&(*n - 2))
    }
}

pub fn fibonacci_iterative(n: &u128) -> u128 {
    if *n <= 1 {
        return *n;
    }
    let mut curr: u128 = 0;
    let mut prev1: u128 = 1;
    let mut prev2: u128 = 0;

    for _ in 2..=*n {
        curr = prev1 + prev2;
        prev2 = prev1;
        prev1 = curr;
    }
    curr
}

pub fn fibonacci_recursive_tail(n: &u128) -> u128 {
    fn f(n: u128, a: u128, b: u128) -> u128 {
        match n {
            0 => a,
            _ => f(n - 1, a + b, a),
        }
    }
    f(*n, 0, 1)
}

pub fn compare_methods() {
    use std::time::Instant;

    let n: u128 = 150;

    let handle = thread::spawn(move || {
        let start_thread = Instant::now();
        let result_thread = fibonacci_recursive_tail(&n);
        let elapsed_thread = start_thread.elapsed();
        println!(
            "Took {elapsed_thread:.2?} to (tail) recursively calculate {n}th fibonacci: {result_thread}"
        );
    });

    let start = Instant::now();
    let result = fibonacci_iterative(&n);
    let elapsed = start.elapsed();
    println!("Took {elapsed:.2?} to iteratively calculate {n}th fibonacci: {result}");
    handle.join().unwrap();
}
