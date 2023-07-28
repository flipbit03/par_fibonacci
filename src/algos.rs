use std::mem::replace;

use num_bigint::{BigUint, ToBigUint};

/// Returns the nth fibonacci number by using an efficient iterative algorithm.
pub fn efficient_fibonacci(n: u128) -> BigUint {
    let mut f0: BigUint = 0.to_biguint().unwrap();
    let mut f1: BigUint = 1.to_biguint().unwrap();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}
