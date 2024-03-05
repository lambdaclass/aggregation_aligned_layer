#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let fib_steps: u32 = env::read();

    let n_fib = fibonacci(fib_steps);

    env::commit(&n_fib);
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 0;
    }

    let mut prev = 0;
    let mut curr = 0;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}
