/// Round floating numbers (f32 or f64)
pub trait RoundFloat {
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
        let number: f64 = 1.455000;
        let result: f64 = number.round_float(decimal_places as i64);
        assert_eq!(result, 1.46);

        let number = 1.455000;
        let result = number.round_float(1);
        assert_eq!(result, 1.5);

        let number = 1.455000;
        let result = number.round_float(0);
        assert_eq!(result, 1.0);

        let number: f32 = -2.0 / 3.0;
        let result: f32 = number.round_float(5); // 5i32
        assert_eq!(result, -0.66667);

        let number: f32 = 5.99997;
        let result: f32 = number.round_float(4); // 4i32
        assert_eq!(result, 6.0); // 6.0000
    ```
    <https://floating-point-gui.de/languages/rust>

    <https://doc.rust-lang.org/std/primitive.f64.html#method.powf>
    */
    fn round_float(self, decimal_places: impl Into<i64>) -> Self
    where
        Self: std::marker::Sized; // This trait is object safe.
}

impl RoundFloat for f64 {
    fn round_float(self, decimal_places: impl Into<i64>) -> f64 {
        let dec: i64 = decimal_places.into();
        if dec <= 0 || self == 0.0 {
            self.round()
        } else {
            let multiplier: f64 = 10.0_f64.powi(dec as i32);
            (self * multiplier).round() / multiplier
        }
    }
}

impl RoundFloat for f32 {
    fn round_float(self, decimal_places: impl Into<i64>) -> f32 {
        let dec: i64 = decimal_places.into();
        if dec <= 0 || self == 0.0 {
            self.round()
        } else {
            let multiplier: f64 = 10.0_f64.powi(dec as i32);
            (((self as f64) * multiplier).round() / multiplier) as f32
        }
    }
}

/*
// https://users.rust-lang.org/t/u32-u64-mapping-revisited
fn convert(num: u64) -> u32 {
    u32::from_ne_bytes(num.to_ne_bytes())
}
*/
