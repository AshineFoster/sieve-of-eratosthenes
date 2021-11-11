mod sieve;
use std::env;
use std::process;

use sieve::LIMIT;

fn main() {
    // input and parse limit from command line

    let limit = env::args().nth(1).unwrap_or_else(|| {
        println!("USAGE:\tsieve_of_eratosthenes NUM");
        println!("ARGS:\t<NUM>\tA number from 2 to {} inclusive.", LIMIT);
        process::exit(1);
    });

    let limit: usize = limit.parse().unwrap_or_else(|err| {
        println!("Error parsing value: {} '{}'", err, limit);
        process::exit(1);
    });

    if !(2..=LIMIT).contains(&limit) {
        println!("Please enter a number from 2 to {} inclusive.", LIMIT);
        process::exit(1);
    }

    println!("Prime numbers:");
    println!("{:?}", sieve::primes(limit));
}
