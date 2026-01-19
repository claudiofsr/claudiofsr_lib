//! # Integer Digits Utility
//!
//! This module provides the [`IntegerDigits`] trait and a standalone [`digit_count`]
//! function to calculate the visual length of integer types.
//!
//! Visual length corresponds to the number of characters used to represent
//! the number as a string (e.g., including the `-` sign for negative numbers).

#[cfg(feature = "decimal")]
use rust_decimal::Decimal;

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
    /// Returns the digit count of the inner value, or `0` if the value is `None`.
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

#[cfg(feature = "decimal")]
impl IntegerDigits for Decimal {
    /// Calculates the visual character count for a [`Decimal`] value.
    ///
    /// ### Logic:
    /// 1. **Sign**: Adds 1 if the number is negative.
    /// 2. **Integral Part**: Counts digits before the decimal point.
    /// 3. **Decimal Point**: Adds 1 if `scale > 0`.
    /// 4. **Fractional Part**: Adds the number of digits defined by `scale`.
    ///
    /// For values between -1 and 1 (exclusive), it ensures at least one `0`
    /// is counted before the decimal point (e.g., `0.05` counts as 4 characters).
    ///
    /// ### Examples:
    /// ```
    /// use claudiofsr_lib::IntegerDigits;
    /// use rust_decimal_macros::dec;
    ///
    /// // Example of what appears in cargo doc:
    /// assert_eq!(dec!(123.45).digit_count(), 6);
    /// assert_eq!(dec!(-0.001).digit_count(), 6);
    /// ```    
    #[inline]
    fn digit_count(self) -> usize {
        let scale = self.scale() as usize;
        let mantissa = self.mantissa(); // i128
        let abs_mantissa = mantissa.unsigned_abs();

        let sign_len = if mantissa < 0 { 1 } else { 0 };

        if scale == 0 {
            // Case 1: Integer-like decimal (e.g., 100)
            if abs_mantissa == 0 {
                1
            } else {
                abs_mantissa.ilog10() as usize + 1 + sign_len
            }
        } else {
            // Case 2: Fractional decimal (e.g., 1.23 or 0.005)
            // Number of digits in the mantissa
            let mantissa_digits = if abs_mantissa == 0 {
                1
            } else {
                abs_mantissa.ilog10() as usize + 1
            };

            // Visual length = Sign + Integral Part + Decimal Point + Fractional Part (scale)
            // If mantissa_digits <= scale, the visual format is "0.xxxxx",
            // so the integral part is always at least 1 digit ('0').
            let visual_digits = std::cmp::max(mantissa_digits, scale + 1);

            visual_digits + 1 + sign_len
        }
    }
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
    #[cfg(feature = "decimal")]
    use rust_decimal_macros::dec; // Optional: for easier decimal creation

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

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal_digit_count() {
        // Integers as Decimals
        assert_eq!(dec!(123).digit_count(), 3);
        assert_eq!(dec!(-123).digit_count(), 4);
        assert_eq!(dec!(0).digit_count(), 1);

        // Decimals with fractional parts
        assert_eq!(dec!(12.34).digit_count(), 5);
        assert_eq!(dec!(-12.34).digit_count(), 6);
        assert_eq!(dec!(0.5).digit_count(), 3);
        assert_eq!(dec!(-0.5).digit_count(), 4);

        // Decimals with leading zeros in fraction
        assert_eq!(dec!(0.001).digit_count(), 5);
        assert_eq!(dec!(-0.001).digit_count(), 6);

        // Decimals with trailing zeros (Decimal preserves scale)
        let d = Decimal::new(100, 2);
        assert_eq!(d.digit_count(), 4); // "1.00"

        let zero_with_scale = Decimal::new(0, 3);
        assert_eq!(zero_with_scale.digit_count(), 5); // "0.000"
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_option_decimal_digit_count() {
        let val_some = Some(dec!(123.45));
        let val_none: Option<Decimal> = None;

        assert_eq!(val_some.digit_count(), 6);
        assert_eq!(val_none.digit_count(), 0);
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_helper_function_with_decimal() {
        assert_eq!(digit_count(dec!(10.10)), 5);
        assert_eq!(digit_count(Some(dec!(-1.1))), 4);
    }
}
