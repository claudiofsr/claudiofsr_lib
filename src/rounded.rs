/// Round floating numbers (f32 or f64)
pub trait RoundFloat<T> {
    /**
    Round floating-point numbers to a specified number of decimal places.

    Two Rounding method for floating-point operations:

    1. Round to nearest value, ties to even:

        if the number falls midway, it is rounded to the nearest value with an even least significant digit.

    2. Round to nearest value, ties away from zero (or ties to away):

        if the number falls midway, it is rounded to the nearest value above (for positive numbers) or below (for negative numbers).

    Python takes the first approach and Rust takes the second.

    Neither is contradicting the IEEE-754 standard, which defines and allows for both.

    Examples:
    ```
        use claudiofsr_lib::RoundFloat;

        let decimal_places: u32 = 2;
        let number: f64 = 1.454999;
        let result: f64 = number.round_float(decimal_places);
        assert_eq!(result, 1.45);

        let decimal_places: usize = 2;
        let result = 1.455000.round_float(decimal_places);
        assert_eq!(result, 1.46);

        let decimal_places: u128 = 6;
        let number: f64 = 3.455000500;
        let result: f64 = number.round_float(decimal_places);
        assert_eq!(result, 3.455001);

        let result = 1.455050.round_float(4); // 4i32
        assert_eq!(result, 1.4551);

        let result = 1.455000.round_float(0);
        assert_eq!(result, 1.0);

        let number: f32 = -2.0 / 3.0;
        let result: f32 = number.round_float(5);
        assert_eq!(result, -0.66667);

        let number: f32 = 5.99997;
        let result: f32 = number.round_float(4);
        assert_eq!(result, 6.0); // 6.0000

        let decimal_places: isize = 3;
        let number: f32 = 5.99997;
        let result: f32 = number.round_float(decimal_places);
        assert_eq!(result, 6.0); // 6.000

        let decimal_places: u8 = 4;
        let result: f32 = 5.00007.round_float(decimal_places);
        assert_eq!(result, 5.0001);
    ```
    <https://floating-point-gui.de/languages/rust>

    <https://doc.rust-lang.org/std/primitive.f64.html#method.powi>

    <https://doc.rust-lang.org/std/convert/trait.TryFrom.html>
    */
    fn round_float(self, decimal_places: T) -> Self
    where
        Self: std::marker::Sized; // This trait is object safe
}

impl<T> RoundFloat<T> for f64
where
    i32: TryFrom<T>,
    <i32 as TryFrom<T>>::Error: std::fmt::Display,
{
    fn round_float(self, decimal_places: T) -> f64 {
        match i32::try_from(decimal_places) {
            Ok(dec) => {
                if dec <= 0 || self == 0.0 {
                    self.round()
                } else {
                    let multiplier: f64 = 10.0_f64.powi(dec);
                    (self * multiplier).round() / multiplier
                }
            }
            Err(why) => {
                let t = std::any::type_name::<T>();
                eprintln!("fn round_float() for f64: {self}");
                eprintln!("Error converting decimal places from type {t} to i32.");
                panic!("Invalid Decimal Places: {why}")
            }
        }
    }
}

impl<T> RoundFloat<T> for f32
where
    i32: TryFrom<T>,
    <i32 as TryFrom<T>>::Error: std::fmt::Display,
{
    fn round_float(self, decimal_places: T) -> f32 {
        match i32::try_from(decimal_places) {
            Ok(dec) => {
                if dec <= 0 || self == 0.0 {
                    self.round()
                } else {
                    let multiplier: f64 = 10.0_f64.powi(dec);
                    (((self as f64) * multiplier).round() / multiplier) as f32
                }
            }
            Err(why) => {
                let t = std::any::type_name::<T>();
                eprintln!("fn round_float() for f32: {self}");
                eprintln!("Error converting decimal places from type {t} to i32.");
                panic!("Invalid Decimal Places: {why}")
            }
        }
    }
}

/// Try Convert Extension
pub trait TryConvertExtension<T> {
    /**
    Try converting type T to type U

    "Simple and safe type conversions that may fail
    in a controlled way under some circumstances.""

    Example:
    ```
        use claudiofsr_lib::TryConvertExtension;

        let type_u8: u8 = 5;
        let type_i16: i16 = 5;
        let type_u32: u32 = 5;
        let type_f64: f64 = 5.0;

        let value_f64: f64 = type_u8.try_convert();
        assert_eq!(type_f64, value_f64);

        let value_u32: u32 = type_i16.try_convert();
        assert_eq!(type_u32, value_u32);

        let value_usize: usize = 7_i32.try_convert();
        assert_eq!(7_usize, value_usize);

        let value_f64: f64 = 9_u16.try_convert();
        assert_eq!(9.0_f64, value_f64);

        // With TurboFish

        let value_u32 = type_i16.try_convert::<u32>();
        assert_eq!(type_u32, value_u32);

        let value_f64 = 2_i32.try_convert::<f64>();
        assert_eq!(2.0_f64, value_f64);
    ```
    */
    fn try_convert<U>(self) -> U
    where
        U: TryFrom<T>,
        <U as TryFrom<T>>::Error: std::fmt::Display;
}

impl<T> TryConvertExtension<T> for T {
    fn try_convert<U>(self) -> U
    where
        U: TryFrom<T>,
        <U as TryFrom<T>>::Error: std::fmt::Display,
    {
        match U::try_from(self) {
            Ok(type_u) => type_u,
            Err(why) => {
                let t = std::any::type_name::<T>();
                let u = std::any::type_name::<U>();
                panic!("Error converting from {t} to {u}: {why}")
            }
        }
    }
}

#[cfg(test)]
mod round_numbers {
    use super::*;

    // cargo test -- --help
    // cargo test -- --nocapture
    // cargo test -- --show-output

    #[test]
    /// `cargo test -- --show-output round_float_f64`
    fn round_float_f64() {
        let decimal_places: u32 = 2;
        let number: f64 = 1.454999;
        let result: f64 = number.round_float(decimal_places);
        assert_eq!(result, 1.45);

        let decimal_places: usize = 3;
        let result = 1.455500.round_float(decimal_places);
        assert_eq!(result, 1.456);

        let decimal_places: u128 = 6;
        let number: f64 = 3.455000500;
        let result: f64 = number.round_float(decimal_places);
        assert_eq!(result, 3.455001);

        let result = 1.455000.round_float(1); // 1i32
        assert_eq!(result, 1.5);

        let result = 1.455000.round_float(0);
        assert_eq!(result, 1.0);

        let number: f64 = 5.99997000 + 4.0e-8;
        let decimal_places: u8 = 255;
        let result: f64 = number.round_float(decimal_places);
        assert_eq!(result, 5.99997004);

        let decimal_places: i32 = -1;
        let result = 1.455000.round_float(decimal_places);
        assert_eq!(result, 1.0);

        let decimal_places: u8 = 255; // type `u8` has range `0..=255`
        let number: f64 = 5.99997000 + 4.0e-8;
        let result: f64 = number.round_float(decimal_places);

        println!("decimal_places: {decimal_places}");
        println!("number: {number:.30}");
        println!("result: {result:.30}");

        assert_eq!(result, 5.99997004);
    }

    #[test]
    fn round_float_f64_negative() {
        let number: f64 = -2.0 / 3.0;
        let result: f64 = number.round_float(5);
        assert_eq!(result, -0.66667);

        let decimal_places: usize = 3;
        let result = (-1.455000).round_float(decimal_places);
        assert_eq!(result, -1.455);
    }

    #[test]
    fn round_float_f64_zero() {
        let number: f64 = 0.0;
        let decimal_places: u64 = 2;
        let result: f64 = number.round_float(decimal_places);
        assert_eq!(result, 0.0);

        let decimal_places: usize = 3;
        let result = 0.000.round_float(decimal_places);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn round_float_f64_nan() {
        let number: f64 = f64::NAN;
        let decimal_places: u32 = 2;
        let result: f64 = number.round_float(decimal_places);
        assert!(result.is_nan());

        let decimal_places: isize = 3;
        let result = f64::NAN.round_float(decimal_places);
        assert!(result.is_nan());
    }

    #[test]
    fn round_float_f64_inf() {
        let number: f64 = f64::INFINITY;
        let decimal_places: u32 = 2;
        let result: f64 = number.round_float(decimal_places);
        assert!(result.is_infinite());

        let decimal_places: usize = 3;
        let result = f64::INFINITY.round_float(decimal_places);
        assert!(result.is_infinite());
    }
}
