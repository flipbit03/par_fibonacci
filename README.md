## "Parallel Fibonacci": Break down calculation by threads.

This is just an exploration in Rust's easy multithreading capabilities (aka "fearless Concurrency"). The Fibonacci algorithm **is not parallelizable**.

Even though we are breaking down the calculation into smaller, parallel chunks, we are actually not gaining any performance, as each thread in the parallel calculation tree ends up re-calculating several of the same Fibonacci numbers. This is basically a glorified CPU stress test ;)

### Usage

    cargo run 400000 32

### Example output

    Calculating Fib(400000) with 32 cores [tree_size=32]
    Fib(400000) => ....15033346875
    Fib(400000) has 83595 digits
    Took 12.577 seconds