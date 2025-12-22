/// A trait to extend integer types with visual digit counting capabilities.
pub trait IntegerDigits {
    /// Returns the number of characters required to represent the integer as a string.
    ///
    /// This includes the negative sign for signed integers.
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
                        // ilog10 returns the exponent of the largest power of 10 <= self.
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
                        // Use unsigned_abs to safely handle T::MIN and avoid ilog10 panics.
                        self.unsigned_abs().ilog10() as usize + 1 + prefix
                    }
                }
            }
        )*
    };
}

impl_unsigned_digit_count!(u8, u16, u32, u64, u128, usize);
impl_signed_digit_count!(i8, i16, i32, i64, i128, isize);

/// A generic wrapper to get the digit count of any type implementing [`IntegerDigits`].
///
/// # Examples
///
/// ```
/// use claudiofsr_lib::digit_count;
///
/// let count = digit_count(-123i32);
/// assert_eq!(count, 4);
/// ```
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
    fn test_digit_count_signed() {
        assert_eq!(digit_count(-100i32), 4);
        assert_eq!(digit_count(100i32), 3);
        assert_eq!(digit_count(0i32), 1);
        assert_eq!(digit_count(i32::MIN), 11);
        assert_eq!(digit_count(i32::MAX), 10);
        assert_eq!(digit_count(-1i8), 2);
        assert_eq!(digit_count(5i8), 1);
    }

    #[test]
    fn test_digit_count_unsigned() {
        assert_eq!(digit_count(5000u16), 4);
        assert_eq!(digit_count(0u64), 1);
        assert_eq!(digit_count(10u8), 2);
        assert_eq!(digit_count(u64::MAX), 20);
    }

    #[test]
    fn test_digit_count_various_types() {
        let values: Vec<i64> = vec![-10, 0, 10, 1000];
        let results: Vec<usize> = values.into_iter().map(digit_count).collect();
        assert_eq!(results, vec![3, 1, 2, 4]);
    }
}
