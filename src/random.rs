use crate::MyResult;
use std::{
    cell::RefCell,
    time::{SystemTime, UNIX_EPOCH},
};
// use std::hash::{BuildHasher, Hasher, RandomState};

/// A simple, fast Pseudo-Random Number Generator (PRNG) using the xorshift* algorithm.
/// This provides better statistical properties than a simple Linear Congruential Generator (LCG).
pub struct XorShiftRng {
    state: u64,
}

impl XorShiftRng {
    // Constructor to create a new instance with a seed
    fn new(seed: u64) -> Self {
        XorShiftRng { state: seed }
    }

    /// Generates the next random u64 number in the sequence.
    fn generate(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12; // a
        x ^= x << 25; // b
        x ^= x >> 27; // c
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
}

/// Provides a seed based on the system's high-resolution clock.
/// This is a good source for a non-deterministic seed.
fn get_seed() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards, system clock is unreliable")
        .as_nanos() as u64;

    // Ensure the seed is never zero, which is invalid for XorShiftRng.
    if nanos == 0 { 1 } else { nanos }
}

// Use a thread-local static RNG. This is the most important best practice.
// 1. `thread_local!`: Creates a variable that is unique to each thread, avoiding expensive locking.
// 2. `RefCell`: Allows us to get a mutable reference (`&mut`) to the RNG from an immutable context.
// This ensures that we use ONE generator per thread and properly advance its state,
// instead of creating and seeding a new one for every random number.
thread_local!(
    static THREAD_RNG: RefCell<XorShiftRng> = RefCell::new(XorShiftRng::new(get_seed()));
);

/// Generate random numbers without external dependencies
pub fn rand() -> u64 {
    // RandomState::new().build_hasher().finish()
    THREAD_RNG.with(|rng| rng.borrow_mut().generate())
}

/// Generates a random integer within a given range `[min, max]` (inclusive).
///
/// This implementation is carefully designed to avoid "modulo bias". It does this using
/// rejection sampling. In the astronomically rare case that it fails to find an
/// unbiased number after a set number of retries, it falls back to a potentially
/// biased result to guarantee termination.
///
/// ### Arguments
/// * `min` - The lower bound of the range (inclusive).
/// * `max` - The upper bound of the range (inclusive).
///
/// ### Errors
/// Returns an error if `min > max`.
pub fn random_in_range(min: u64, max: u64) -> MyResult<u64> {
    if min > max {
        let msg = format!("min ({min}) must be less than or equal to max ({max})");
        return Err(msg.into());
    }

    // The number of possible outcomes in the range [min, max].
    // `wrapping_add(1)` correctly handles the case where the range is the full `u64`.
    // In that case, `max - min` is `u64::MAX`, and `wrapping_add(1)` results in 0.
    let range_size = max.wrapping_sub(min).wrapping_add(1);

    // If range_size is 0, it signifies the full u64 range was requested.
    if range_size == 0 {
        return Ok(rand());
    }

    // To avoid modulo bias, we find the largest multiple of `range_size` that
    // fits in a u64. Any random number generated above this threshold would,
    // if mapped with modulo, create an unfair distribution.
    let rejection_threshold = (u64::MAX / range_size) * range_size;

    // The number of attempts before falling back to a biased result.
    // The probability of exceeding this is negligible.
    const MAX_RETRIES: u32 = 100;

    for _ in 0..MAX_RETRIES {
        let value = rand();
        // If the value is within the unbiased zone, we use it. This is the common path.
        if value < rejection_threshold {
            return Ok(min + (value % range_size));
        }
        // Otherwise, we "reject" the sample and try again.
    }

    // Fallback: If we exhausted all retries (extremely unlikely), we return a
    // result that may be slightly biased. This guarantees function termination.
    Ok(min + (rand() % range_size))
}

/// A trait for shuffling mutable slices in place.
///
/// This trait provides a `shuffle` method that shuffles the elements of any mutable
/// slice `&mut [T]` using the modern Fisher-Yates algorithm (Durstenfeld's variant).
pub trait Shuffle {
    /// Shuffles the elements of the slice in place.
    ///
    /// This algorithm iterates from the end of the slice to the beginning, swapping
    /// each element with a randomly selected element from the part of the slice
    /// that has not yet been shuffled.
    ///
    /// ### Examples
    ///
    /// ```
    /// use claudiofsr_lib::Shuffle;
    ///
    /// let mut strings = vec!["abc", "foo", "bar", "baz", "mm nn", "zzz"];
    /// strings.shuffle();
    /// println!("shuffled strings: {:?}", strings);
    ///
    /// let mut integers: Vec<u32> = (1..=20).collect();
    /// integers.shuffle();
    /// println!("shuffled integers: {:?}", integers);
    /// ```
    ///
    /// ### Links
    /// - <https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle>
    /// - <https://stackoverflow.com/questions/26033976/how-do-i-create-a-vec-from-a-range-and-shuffle-it>
    fn shuffle(&mut self);
}

// Implement the Shuffle trait for any mutable slice of any type T.
impl<T> Shuffle for &mut [T] {
    fn shuffle(&mut self) {
        let len = self.len();
        // The loop `(1..len).rev()` handles slices of length 0 or 1 gracefully
        // by not executing, so a separate `if len > 1` check is not needed.
        for i in (1..len).rev() {
            // The algorithm swaps the element at index `i` with an element at a
            // randomly chosen index `j` from the range `[0, i]` (inclusive).

            // Generate a random index `j` such that `0 <= j <= i`.
            // The call to `.unwrap()` here is guaranteed to be safe and will never panic.
            // This is because `random_in_range(min, max)` only returns an `Err`
            // if `min > max`. In this loop, the arguments are `random_in_range(0, i)`,
            // and the loop invariant ensures that `i` is always >= 1.
            // Therefore, `0 <= i` is always true, the function will always return `Ok(...)`,
            // and the unwrap is safe.
            let j = random_in_range(0, i as u64).unwrap() as usize;

            self.swap(i, j);
        }
    }
}

// Implement the Shuffle trait for Vec<T> by delegating to its slice.
impl<T> Shuffle for Vec<T> {
    fn shuffle(&mut self) {
        self.as_mut_slice().shuffle();
    }
}

#[cfg(test)]
mod test_random {
    use super::*; // Import everything from the parent module
    use std::collections::HashSet; // Import HashSet for tests

    #[test]
    /// Tests that the thread-local RNG is stateful and produces different numbers.
    ///
    /// `cargo test -- --show-output gen_random`
    fn gen_random() {
        let mut numbers = HashSet::new();

        // Generate and print some random numbers
        for n in 0..1000 {
            let random = rand();
            println!("random number {n:3}: {random}");

            // Check for a specific one.
            if !numbers.insert(random) {
                eprintln!("Error: {random}");
                panic!("Not random!");
            }
        }

        // It's astronomically unlikely that 1000 random u64s will have a collision.
        // A failure here would indicate the RNG state is not advancing.
        println!("numbers: {numbers:#?}");
        assert_eq!(numbers.len(), 1000);
    }

    #[test]
    /// Verifies that the shuffle function correctly permutes all elements.
    ///
    /// `cargo test -- --show-output shuffle_preserves_elements`
    fn shuffle_preserves_elements() {
        let mut original: Vec<u32> = (1..=100).collect();
        let mut shuffled = original.clone();

        shuffled.shuffle(); // Using the trait method

        println!("original: {original:?}");
        println!("shuffled: {shuffled:?}");

        // Basic checks: length should be the same, and order should be different.
        assert_eq!(original.len(), shuffled.len());
        assert_ne!(
            original, shuffled,
            "Shuffle should change the order (highly likely)."
        );

        // The most important check: a shuffled vector must contain exactly the same elements.
        // We can verify this by sorting both and comparing.
        original.sort();
        shuffled.sort();
        assert_eq!(
            original, shuffled,
            "A valid shuffle must preserve all original elements."
        );
    }

    #[test]
    /// Tests that generated values fall within the specified inclusive range.
    fn random_in_range_bounds() -> MyResult<()> {
        let min = 100;
        let max = 200;
        for _ in 0..10_000 {
            let val = random_in_range(min, max)?;
            assert!(
                val >= min && val <= max,
                "Value {val} is outside the range [{min}, {max}]"
            );
        }
        Ok(())
    }

    #[test]
    /// `cargo test -- --show-output random_integers`
    ///
    /// <https://stackoverflow.com/questions/48218459/how-do-i-generate-a-vector-of-random-numbers-in-a-range>
    fn random_integers() -> MyResult<()> {
        // Example: Get a random integer value in the range 1 to 20:
        let value: u64 = random_in_range(1, 20)?;

        println!("integer: {value:?}");

        // Generate a vector of 100 64-bit integer values in the range from 1 to 20,
        // allowing duplicates:

        let integers: Vec<u64> = (0..100)
            .map(|_| random_in_range(1, 20))
            .collect::<Result<Vec<u64>, _>>()?;

        println!("integers: {integers:?}");

        let condition_a = integers.iter().min() >= Some(&1);
        let condition_b = integers.iter().max() <= Some(&20);

        assert!(condition_a);
        assert!(condition_b);
        assert_eq!(integers.len(), 100);

        Ok(())
    }

    #[test]
    /// Ensures that an invalid range (min > max) correctly returns an error.
    ///
    /// `cargo test -- --show-output random_in_range_errors_on_invalid_range`
    fn random_in_range_errors_on_invalid_range() -> MyResult<()> {
        let result = random_in_range(21, 20).map_err(|err| {
            eprintln!("{err}");
            err
        });
        assert!(result.is_err());

        // We can also check the specific error message for a more robust test.
        assert_eq!(
            result.unwrap_err().to_string(),
            "min (21) must be less than or equal to max (20)"
        );

        Ok(())
    }

    #[test]
    /// Tests the `shuffle` method on empty and single-element vectors.
    fn shuffle_empty_and_single_element() {
        let mut empty_vec: Vec<u32> = Vec::new();
        empty_vec.shuffle();
        assert!(empty_vec.is_empty());

        let mut single_vec = vec![42];
        single_vec.shuffle();
        assert_eq!(single_vec, vec![42]); // Single element remains unchanged
    }

    #[test]
    /// Tests that calling shuffle multiple times produces different results (high probability).
    fn shuffle_multiple_times() {
        let original: Vec<u32> = (1..=10).collect();
        let mut first_shuffle = original.clone();
        first_shuffle.shuffle();

        let mut second_shuffle = original.clone();
        second_shuffle.shuffle();

        // While not guaranteed, it's astronomically unlikely that two independent shuffles
        // of the same initial 10 elements will produce the exact same sequence.
        assert_ne!(first_shuffle, original);
        assert_ne!(second_shuffle, original);
        assert_ne!(
            first_shuffle, second_shuffle,
            "Two shuffles produced the same result, which is highly unlikely."
        );
    }
}
