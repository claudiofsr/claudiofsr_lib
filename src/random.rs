use crate::MyResult;
use std::hash::{BuildHasher, Hasher, RandomState};
/**
Shuffle the vector in place with the Fisher-Yates algorithm.

```
    use claudiofsr_lib::shuffle;

    let mut strings = vec!["abc", "foo", "bar", "baz", "mm nn", "zzz"];

    shuffle(&mut strings);

    println!("strings: {:?}", strings);

    let mut integers: Vec<u32> = (1..=20).collect();

    shuffle(&mut integers);

    println!("integers: {:?}", integers);
```

<https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle>

<https://stackoverflow.com/questions/26033976/how-do-i-create-a-vec-from-a-range-and-shuffle-it>

*/
pub fn shuffle<T>(vec: &mut [T]) {
    let n: usize = vec.len();
    for i in 0..(n - 1) {
        // Generate random index j, such that: i <= j < n
        // The remainder (`%`) after division is always less than the divisor.
        let j = (rand() as usize) % (n - i) + i;
        vec.swap(i, j);
    }
}

/// Generate a random integer value in the given range (min, max) inclusive.
pub fn get_random_integer(min: u64, max: u64) -> u64 {
    min + rand() % (max - min + 1)
}

/// Generate a random integer value in the given range (min, max) inclusive.
///
/// Return error if `min > max``
pub fn get_random_integer_v2(min: u64, max: u64) -> MyResult<u64> {
    if min > max {
        let msg = format!("min ({min}) must be less than or equal to max ({max})");
        Err(msg.into())
    } else {
        // The remainder (`%`) after division is always less than the divisor.
        Ok(min + rand() % (max - min + 1))
    }
}

/// Generate random numbers without external dependencies
pub fn rand() -> u64 {
    RandomState::new().build_hasher().finish()
}

#[cfg(test)]
mod test_random {
    use crate::*;

    #[test]
    /// `cargo test -- --show-output vec_shuffle`
    fn vec_shuffle() {
        let mut vec: Vec<u32> = (1..=100).collect();
        shuffle(&mut vec);

        println!("vec: {:?}", vec);
        assert_eq!(vec.len(), 100);
    }

    #[test]
    /// `cargo test -- --show-output random_integers_v1`
    ///
    /// <https://stackoverflow.com/questions/48218459/how-do-i-generate-a-vector-of-random-numbers-in-a-range>
    fn random_integers_v1() {
        // Example: Get a random integer value in the range 1 to 20:
        let value: u64 = get_random_integer(1, 20);

        println!("integer: {:?}", value);

        // Generate a vector of 100 64-bit integer values in the range from 1 to 20,
        // allowing duplicates:

        let integers: Vec<u64> = (0..100).map(|_| get_random_integer(1, 20)).collect();

        println!("integers: {:?}", integers);

        let condition_a = integers.iter().min() >= Some(&1);
        let condition_b = integers.iter().max() <= Some(&20);

        assert!(condition_a);
        assert!(condition_b);
        assert_eq!(integers.len(), 100);
    }

    #[test]
    /// `cargo test -- --show-output random_integers_v2`
    ///
    /// <https://stackoverflow.com/questions/48218459/how-do-i-generate-a-vector-of-random-numbers-in-a-range>
    fn random_integers_v2() -> MyResult<()> {
        // Example: Get a random integer value in the range 1 to 20:
        let value: u64 = get_random_integer_v2(1, 20)?;

        println!("integer: {:?}", value);

        // Generate a vector of 100 64-bit integer values in the range from 1 to 20,
        // allowing duplicates:

        let integers: Vec<u64> = (0..100)
            .map(|_| get_random_integer_v2(1, 20))
            .collect::<Result<Vec<u64>, _>>()?;

        println!("integers: {:?}", integers);

        let condition_a = integers.iter().min() >= Some(&1);
        let condition_b = integers.iter().max() <= Some(&20);

        assert!(condition_a);
        assert!(condition_b);
        assert_eq!(integers.len(), 100);

        Ok(())
    }

    #[test]
    /// `cargo test -- --show-output random_integers_v3`
    fn random_integers_v3() -> MyResult<()> {
        let result = get_random_integer_v2(21, 20).map_err(|err| {
            eprintln!("{err}");
            err
        });
        assert!(result.is_err());

        let error = result.unwrap_err();
        eprintln!("error: {error:?}");

        Ok(())
    }
}
