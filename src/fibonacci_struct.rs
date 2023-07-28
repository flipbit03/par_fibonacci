use std::thread;

use num_bigint::BigUint;

use crate::algos::efficient_fibonacci;

/// Tree-like Structure to represent the set of calculations to be performed to reach the
/// final result.
///
/// A calculation of fib(50) can be represented by:
/// - Calculating fib(50) in a single go.
/// - (or) Calculating fib(49) and fib(48) in parallel, and then summing the results.
///   - conversely, the first (fib(49)) can also be broken down into (fib(48), fib(47)) in parallel
///   - the second one (fib(48)) can also be broken down into (fib(47), fib(46)) in parallel
///   - and so on...
///
/// By applying the logic above, we can keep breaking down the calculation into
/// smaller and smaller pairs until we reach a point where the amount of parallel calculations enables
/// us to use all the cores in the system efficiently.
#[derive(Debug, Clone)]
pub enum Fibonacci {
    // Edge: Fibonacci as a single calculation
    Number(u128),
    // Node: Fibonacci as a pair of calculations, which can possibly be pairs themselves.
    Pair(Box<Fibonacci>, Box<Fibonacci>),
}

impl Fibonacci {
    /// Create a new Fibonacci tree with the given number and the maximum tree size.
    ///
    /// The maximum tree size parameter is the (desired) number of nodes in the tree, so that we can
    /// offload the calculations to all the cores in the system.
    pub fn new(n: u128, max_tree_size: u128) -> Self {
        match max_tree_size <= 1 {
            true => Self::Number(n),
            false => {
                let (left, right) = (n - 1, n - 2);
                // Recursively break down the numbers into smaller and smaller pairs,
                // and halve the max_tree_size as we go down.
                Self::Pair(
                    Box::new(Self::new(left, max_tree_size / 2)),
                    Box::new(Self::new(right, max_tree_size / 2)),
                )
            }
        }
    }

    // Calculate the number of nodes in the tree.
    pub fn tree_size(&self) -> u128 {
        match self {
            Self::Number(_) => 1,
            Self::Pair(left, right) => left.tree_size() + right.tree_size(),
        }
    }

    /// Calculate the fibonacci number represented by this tree.
    pub fn calculate(&self) -> BigUint {
        match self {
            // If we have to calculate fibonacci from a single number,
            // just go ahead and do it in the current thread.
            Self::Number(n) => efficient_fibonacci(*n),
            // If we have to calculate fibonacci from a pair of numbers,
            // calculate them in parallel and sum the results.
            // this will drill down in a massively parallel fashion until we reach the edge (Self::Number) cases.
            Self::Pair(left, right) => thread::scope(|s| {
                let t1 = s.spawn(|| left.calculate());
                let t2 = s.spawn(|| right.calculate());

                // Wait for the threads to finish and sum the results.
                t1.join().unwrap() + t2.join().unwrap()
            }),
        }
    }
}
