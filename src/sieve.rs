/// The upper bound on the range of values you can find primes in.
pub const LIMIT: usize = 8_000_000;

pub fn primes(limit: usize) -> Vec<usize> {
    // create a array of integers
    let mut values: [u8; LIMIT] = [1; LIMIT];
    // set value as composite if it is a multiple of another number
    set_composite(&mut values[0..limit]);
    // gets the prime numbers
    get_primes(&values[0..limit])
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
    values
        .iter()
        .enumerate()
        .filter(|(_, value)| **value == 1)
        .map(|(index, _)| index + 1)
        .collect()
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
