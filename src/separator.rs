use std::fmt::{self, Write};

#[cfg(feature = "decimal")]
use rust_decimal::{Decimal, RoundingStrategy};

// ============================================================================
// thousands_separator - High Performance Version
// ============================================================================

/// Defines localized formatting styles for separators.
pub enum FormatStyle {
    /// 1.234,56 (Common in Europe and South America)
    Euro,
    /// 1.234,56 (Same as Euro, specifically for Brazilian context)
    PtBr,
    /// 1,234.56 (Standard in US, UK, and International science)
    Us,
}

/// Unifies numeric types and provides a high-performance writing interface.
///
/// Instead of returning a new `String` (which causes a heap allocation),
/// this trait uses a mutable buffer to write data in-place.
pub trait FormattableNumber {
    /// Checks if the value is negative.
    fn is_negative_num(&self) -> bool;

    /// Formats the absolute value into the provided buffer.
    /// Returns `fmt::Result` to comply with the `Write` trait.
    fn write_abs(&self, decimals: usize, buf: &mut String) -> fmt::Result;
}

/// Macro to implement formatting for floating-point types efficiently.
macro_rules! impl_formattable_float {
    ($t:ty) => {
        impl FormattableNumber for $t {
            fn is_negative_num(&self) -> bool {
                self.is_sign_negative()
            }
            fn write_abs(&self, decimals: usize, buf: &mut String) -> fmt::Result {
                // write! appends directly to the buffer, avoiding a temporary String allocation.
                write!(buf, "{:.1$}", self.abs(), decimals)
            }
        }
    };
}

impl_formattable_float!(f32);
impl_formattable_float!(f64);

#[cfg(feature = "decimal")]
impl FormattableNumber for Decimal {
    fn is_negative_num(&self) -> bool {
        self.is_sign_negative()
    }
    fn write_abs(&self, decimals: usize, buf: &mut String) -> fmt::Result {
        // High-precision rounding specifically for financial/decimal types.
        let rounded = self
            .abs()
            .round_dp_with_strategy(decimals as u32, RoundingStrategy::MidpointNearestEven);
        write!(buf, "{:.1$}", rounded, decimals)
    }
}

/// Blanket implementation for references.
/// Allows the function to accept `&f64` or `f64` interchangeably.
impl<T: FormattableNumber> FormattableNumber for &T {
    fn is_negative_num(&self) -> bool {
        (*self).is_negative_num()
    }
    fn write_abs(&self, decimals: usize, buf: &mut String) -> fmt::Result {
        (*self).write_abs(decimals, buf)
    }
}

/// Implementation for `Option<T>`.
/// If the value is `None`, it defaults to zero formatting.
impl<T: FormattableNumber> FormattableNumber for Option<T> {
    fn is_negative_num(&self) -> bool {
        // is_some_and is highly efficient as it avoids unnecessary pattern matching.
        self.as_ref().is_some_and(|v| v.is_negative_num())
    }
    fn write_abs(&self, decimals: usize, buf: &mut String) -> fmt::Result {
        match self {
            Some(val) => val.write_abs(decimals, buf),
            None => write!(buf, "{:.1$}", 0.0, decimals),
        }
    }
}

/**
Formats numeric values into localized strings with thousands separators and custom decimal precision.

This function supports various numeric types (`f32`, `f64`, `Decimal`) via the `FormattableNumber` trait
and applies formatting based on the selected `FormatStyle`.

### Features
* **High Performance**: Optimized with a "Single Allocation" strategy. It calculates the exact required
  memory upfront to minimize heap allocations and avoid reallocations.
* **Polymorphic Support**: Works with `f32`, `f64`, `Decimal` (optional), and their references.
* **Smart Option Handling**: Directly accepts `Option<T>`, treating `None` as zero while maintaining consistent formatting.
* **Localization**: Supports multiple geographical styles (e.g., Brazilian/European `1.234,56` vs. US `1,234.56`).
* **Precise Rounding**: Applies standard formatting for floats and Midpoint-Nearest-Even (Banker's Rounding) for `Decimal`.

### Arguments
* `value` - The numeric value to format (implements `FormattableNumber`).
* `decimals` - The number of decimal places to include in the output.
* `style` - The `FormatStyle` determining the thousands and decimal separators.

### Example

```rust
    use claudiofsr_lib::{thousands_separator, FormatStyle};

    let number = 1234567.8952;

    // Brazilian format: Dot for thousands, Comma for decimals
    let pt_br = thousands_separator(number, 2, FormatStyle::PtBr);
    assert_eq!(pt_br, "1.234.567,90"); // Note the rounding

    // US format: Comma for thousands, Dot for decimals
    let us = thousands_separator(number, 3, FormatStyle::Us);
    assert_eq!(us, "1,234,567.895");
```
*/
pub fn thousands_separator<T: FormattableNumber>(
    value: T,
    decimals: usize,
    style: FormatStyle,
) -> String {
    // A. Temporary Buffer: Stores the absolute raw formatted number (e.g., "1234.56").
    // Heuristic: 20 digits for the integer part + decimal places.
    let temp_capacity = 20 + decimals;
    let mut abs_temp = String::with_capacity(temp_capacity);

    // B. Writing to String is infallible (it only fails if OOM), so we ignore the Result.
    // This avoids the overhead/bloat of `unwrap()` while keeping the code safe.
    let _ = value.write_abs(decimals, &mut abs_temp);

    // C. Selection of localized separators.
    let (thousands_sep, decimal_sep) = match style {
        FormatStyle::Euro | FormatStyle::PtBr => ('.', ","),
        FormatStyle::Us => (',', "."),
    };

    // D. Logic to split integer and fraction.
    // split_once is O(n) and returns references (&str), creating no new strings.
    let (integer_part, fraction_part) = match abs_temp.split_once('.') {
        Some((i, f)) if decimals > 0 => (i, Some(f)),
        _ => (abs_temp.as_str(), None),
    };

    // E. CAPACITY CALCULATION:
    let is_neg = value.is_negative_num();
    // (len - 1) / 3 gives the exact number of separators needed.
    let num_seps = integer_part.len().saturating_sub(1) / 3;

    // We sum: raw_len + (seps * sep_bytes) + (1 if negative).
    let final_capacity = abs_temp.len() + (num_seps * thousands_sep.len_utf8()) + (is_neg as usize);

    // F. SINGLE ALLOCATION: Allocate the exact amount of RAM needed.
    let mut result = String::with_capacity(final_capacity);

    // G. SEQUENTIAL WRITING: Appending to a pre-allocated String is extremely fast (O(n)).
    if is_neg {
        result.push('-');
    }

    add_sep(integer_part, thousands_sep, &mut result);

    if let Some(f) = fraction_part {
        result.push_str(decimal_sep);
        result.push_str(f);
    }

    result
}

/// Fast helper to insert thousands separators.
///
/// It iterates over bytes because numeric strings are guaranteed to be ASCII.
/// Iterating bytes is faster than iterating Unicode chars as it skips UTF-8 validation.
pub fn add_sep(integer: &str, separator: char, buffer: &mut String) {
    let len = integer.len();
    if len == 0 {
        return;
    }

    for (i, &byte) in integer.as_bytes().iter().enumerate() {
        // Insert separator if the remaining digits are a multiple of 3.
        if i > 0 && (len - i).is_multiple_of(3) {
            buffer.push(separator);
        }
        buffer.push(byte as char);
    }
}

//----------------------------------------------------------------------------//
//                                   Tests                                    //
//----------------------------------------------------------------------------//
//
// cargo test -- --help
// cargo test -- --nocapture
// cargo test -- --show-output
// cargo test --features decimal -- --show-output separator_tests

#[cfg(test)]
mod separator_tests {
    use super::*;

    #[test]
    fn thousands_separator_test() {
        // Test with reference of f32 (&f32)
        let val_f32: &f32 = &-5000.0;
        let result = thousands_separator(val_f32, 2, FormatStyle::PtBr);
        println!("f32: {val_f32}");
        println!("result: {result}\n");
        assert_eq!(result, "-5.000,00");

        // Test with f64
        let val_f64: f64 = -1234567.8949;
        let result = thousands_separator(val_f64, 2, FormatStyle::PtBr);
        println!("f64: {val_f64}");
        println!("result: {result}\n");
        assert_eq!(result, "-1.234.567,89");

        // Test with f64 rounding
        let val_f64_round: f64 = -1234567.8950;
        let result = thousands_separator(val_f64_round, 2, FormatStyle::PtBr);
        println!("f64: {val_f64_round}");
        println!("result: {result}\n");
        assert_eq!(result, "-1.234.567,90");

        // Test US style
        let val_us = 1234567.8912;
        let result = thousands_separator(val_us, 2, FormatStyle::Us);
        println!("f64: {val_us}");
        println!("us result: {result}\n");
        assert_eq!(result, "1,234,567.89");
    }

    /// New tests for Option<T> types
    #[test]
    fn test_options() {
        // Test Some(value)
        let val_some: Option<f64> = Some(1234.567);
        let result = thousands_separator(val_some, 2, FormatStyle::PtBr);
        println!("val_some: {val_some:?}");
        println!("result: {result}\n");
        assert_eq!(result, "1.234,57");

        // Test None
        let val_none: Option<f64> = None;
        let result = thousands_separator(val_none, 2, FormatStyle::Us);
        println!("val_none: {val_none:?}");
        println!("result: {result}\n");
        assert_eq!(result, "0.00");

        // Test None with 0 decimals
        let result_no_dec = thousands_separator(val_none, 0, FormatStyle::Euro);
        assert_eq!(result_no_dec, "0");
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal() {
        let val_decimal = Decimal::new(12345678850, 4); // 1234567.8850
        let result = thousands_separator(val_decimal, 2, FormatStyle::PtBr);
        println!("decimal: {val_decimal}");
        println!("result: {result}\n");
        // MidpointNearestEven: 0.885 -> 0.88
        assert_eq!(result, "1.234.567,88");

        let val_decimal = Decimal::new(12345678950, 4); // 1234567.8950
        let result = thousands_separator(val_decimal, 2, FormatStyle::PtBr);
        println!("decimal: {val_decimal}");
        println!("result: {result}\n");
        // MidpointNearestEven: 0.895 -> 0.90
        assert_eq!(result, "1.234.567,90");

        let val_decimal: Decimal = Decimal::new(12345678951, 4); // 1234567.8951
        let result = thousands_separator(val_decimal, 3, FormatStyle::PtBr);
        println!("decimal: {val_decimal}");
        println!("result: {result}\n");
        assert_eq!(result, "1.234.567,895");

        // Test 1 with Option<Decimal>
        let opt_decimal: Option<Decimal> = Some(Decimal::new(50000000, 0));
        assert_eq!(
            thousands_separator(opt_decimal, 4, FormatStyle::PtBr),
            "50.000.000,0000"
        );

        // Test 2 with Option<Decimal>
        let opt_decimal: Option<Decimal> = Some(Decimal::new(50000000, 0));
        assert_eq!(
            thousands_separator(opt_decimal, 3, FormatStyle::Us),
            "50,000,000.000"
        );
    }
}
