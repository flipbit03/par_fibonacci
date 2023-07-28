mod fibonacci_struct;
use clap::Parser;
use std::{cmp::min, time::Instant};

use crate::fibonacci_struct::Fibonacci;
mod algos;

#[derive(Parser, Debug)]
struct FibonacciCLIArgs {
    /// fib(n) Number to calculate
    #[arg(default_value_t = 400000)]
    number: u128,

    /// Number of cores to use
    #[arg(default_value_t = 0)]
    core_count: u128,
}

fn main() {
    // Parse the command line arguments.
    let args = FibonacciCLIArgs::parse();

    // If the user didn't specify a core count, use (cpu_cores * 8) in the system.
    let core_count = min(
        match args.core_count {
            0 => {
                let cpu_core_count = num_cpus::get() as u128;
                println!(
                    "No core count specified, using cpu_cores({}) count threads",
                    cpu_core_count
                );
                cpu_core_count
            }
            _ => args.core_count,
        },
        args.number,
    );

    // Create a new Fibonacci tree with the given number and the maximum tree size.
    let fib_tree = Fibonacci::new(args.number, core_count);

    println!(
        "Calculating Fib({}) with {} cores [tree_size={}]",
        args.number,
        core_count,
        fib_tree.tree_size()
    );

    // Calculate and display the fibonacci number represented by this tree.
    // also measure the time it took to calculate.
    let took = {
        let before = Instant::now();

        let result = fib_tree.calculate();

        println!("Fib({}) => {}", args.number, result);
        println!(
            "Fib({}) has {} digits",
            args.number,
            result.to_string().len()
        );
        let after = Instant::now();

        (after - before).as_secs_f32()
    };

    // Display the time it took to calculate.
    println!("Took {} seconds", took);
}
