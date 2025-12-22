//! # Integer Digits Utility
//!
//! This module provides the [`IntegerDigits`] trait and a standalone [`digit_count`]
//! function to calculate the visual length of integer types.
//!
//! Visual length corresponds to the number of characters used to represent
//! the number as a string (e.g., including the `-` sign for negative numbers).

/// A trait to extend integer types and their options with visual digit counting capabilities.
pub trait IntegerDigits {
    /// Returns the number of characters required to represent the integer as a base-10 string.
    ///
    /// This calculation is performed without string allocation, making it highly efficient.
    ///
    /// ### Behavior:
    /// - **Sign:** For signed integers, the negative sign (`-`) is counted as one digit.
    /// - **Zero:** The digit count for `0` is `1`.
    /// - **Option:** For `Option<T>`, `None` returns `0`, while `Some(n)` returns the count of `n`.
    ///
    /// ### Examples
    ///
    /// #### Using Method Syntax
    /// ```
    /// use claudiofsr_lib::IntegerDigits;
    ///
    /// assert_eq!(12345.digit_count(), 5);
    /// assert_eq!((-123).digit_count(), 4);
    /// assert_eq!(0.digit_count(), 1);
    /// assert_eq!(Some(12305).digit_count(), 5);
    /// ```
    fn digit_count(self) -> usize;
}

// Macro to implement digit_count for unsigned integer types.
macro_rules! impl_unsigned_digit_count {
    ($($t:ty),*) => {
        $(
            impl IntegerDigits for $t {
                #[inline]
                fn digit_count(self) -> usize {
                    if self == 0 {
                        1
                    } else {
                        // ilog10 is available since Rust 1.67 and is very fast.
                        self.ilog10() as usize + 1
                    }
                }
            }
        )*
    };
}

// Macro to implement digit_count for signed integer types.
macro_rules! impl_signed_digit_count {
    ($($t:ty),*) => {
        $(
            impl IntegerDigits for $t {
                #[inline]
                fn digit_count(self) -> usize {
                    if self == 0 {
                        1
                    } else {
                        // Account for the '-' sign if the number is negative.
                        let prefix = if self < 0 { 1 } else { 0 };
                        // Use unsigned_abs to safely handle T::MIN and avoid overflow/panic.
                        self.unsigned_abs().ilog10() as usize + 1 + prefix
                    }
                }
            }
        )*
    };
}

impl_unsigned_digit_count!(u8, u16, u32, u64, u128, usize);
impl_signed_digit_count!(i8, i16, i32, i64, i128, isize);

/// Implementation for `Option<T>`.
///
/// Returns `0` if the value is `None`, otherwise returns the digit count of the inner value.
impl<T: IntegerDigits> IntegerDigits for Option<T> {
    #[inline]
    fn digit_count(self) -> usize {
        match self {
            Some(n) => n.digit_count(),
            None => 0,
        }
    }
}

/// A convenience function to count digits.
///
/// This function accepts any type that implements [`IntegerDigits`],
/// including standard integers and `Option<Integer>`.
///
/// ### Examples
///
/// ```
/// use claudiofsr_lib::digit_count;
///
/// assert_eq!(digit_count(-123i32), 4);
/// assert_eq!(digit_count(Some(123456789)), 9);
/// assert_eq!(digit_count(0), 1);
/// assert_eq!(digit_count(None::<u32>), 0);
/// ```
#[inline]
pub fn digit_count<T: IntegerDigits>(n: T) -> usize {
    n.digit_count()
}

//----------------------------------------------------------------------------//
//                                   Tests                                    //
//----------------------------------------------------------------------------//
//
// cargo test -- --help
// cargo test -- --nocapture
// cargo test -- --show-output count_tests

#[cfg(test)]
mod count_tests {
    use super::*;

    #[test]
    fn test_digit_count_method_syntax() {
        assert_eq!((-100i32).digit_count(), 4);
        assert_eq!(100i32.digit_count(), 3);
        assert_eq!(0i32.digit_count(), 1);
        assert_eq!(i32::MIN.digit_count(), 11);
        assert_eq!(5000u16.digit_count(), 4);
        assert_eq!(0u64.digit_count(), 1);

        let val = Some(12345);
        assert_eq!(val.digit_count(), 5);
        assert_eq!(None::<i32>.digit_count(), 0);
    }

    #[test]
    fn test_digit_count_various_types() {
        let values: Vec<i64> = vec![-10, 0, 10, 1000, 12345];
        let results: Vec<usize> = values.into_iter().map(|n| n.digit_count()).collect();
        assert_eq!(results, vec![3, 1, 2, 4, 5]);
    }

    #[test]
    fn test_helper_function() {
        assert_eq!(digit_count(123), 3);
        assert_eq!(digit_count(-123), 4);
        assert_eq!(digit_count(Some(12345)), 5);
        assert_eq!(digit_count(None::<i32>), 0);
    }
}
