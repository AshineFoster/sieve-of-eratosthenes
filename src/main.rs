const LIMIT: usize = 1_000_000;

fn main() {
    // create a array of integers
    let mut values: [u8; LIMIT] = [1; LIMIT];
    // set value as composite if it is a multiple of another number
    set_composite(&mut values);
    // gets the prime numbers
    let primes = get_primes(&values);
    // print values that are prime
    println!("Prime numbers:");
    println!("{:?}", primes);
}

/// Given an array of values returns an array with the prime numbers set to 1 and composite to 0.
fn set_composite(values: &mut [u8]) -> &[u8] {
    values[0] = 0;
    let sqr_val_len = (values.len() as f64).sqrt() as usize;

    for index in 2..=sqr_val_len {
        if values[index - 1] == 1 {
            let mut multiples = index * index;
            while multiples <= values.len() {
                values[multiples - 1] = 0;
                multiples += index;
            }
        }
    }
    values
}

/// returns a vector of the prime numbers in the given array
fn get_primes(values: &[u8]) -> Vec<usize> {
    let mut primes = Vec::new();

    for (index, val) in values.iter().enumerate() {
        if *val == 1 {
            primes.push(index + 1)
        }
    }
    primes
}

#[cfg(test)]
mod test_sieve {

    use super::*;

    const NUMTO10: [u8; 10] = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0];

    #[test]
    fn test_set_composite() {
        let mut test_array: [u8; 100] = [1; 100];
        assert_eq!(set_composite(&mut test_array[0..10]), NUMTO10);
    }

    #[test]
    fn test_get_primes() {
        assert_eq!(get_primes(&NUMTO10), vec![2, 3, 5, 7]);
    }
}
