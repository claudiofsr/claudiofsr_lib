use std::fmt::Display;

/// Combine two Options with one operation.
pub trait OptionExtension<T> {
    /**
    Combine two Options with one operation.
    ```
    use claudiofsr_lib::OptionExtension;

    let a = Some(5.0);
    let b = Some(10.0);

    let sum = |a, b| {a + b};
    let sub = |a, b| {a - b};
    let mul = |a, b| {a * b};
    let div = |a, b| {a / b};

    let result_sum = a.combine_with(b, sum);
    let result_sub = a.combine_with(b, sub);
    let result_mul = a.combine_with(b, mul);
    let result_div = a.combine_with(b, div);

    assert_eq!(result_sum, Some(15.0));
    assert_eq!(result_sub, Some(-5.0));
    assert_eq!(result_mul, Some(50.0));
    assert_eq!(result_div, Some(0.5));
    ```
    <https://stackoverflow.com/questions/33779562/is-there-any-built-in-way-to-combine-two-options>
    <https://docs.rs/stdext/latest/stdext/option/trait.OptionExt.html>
    */
    fn combine_with<U, R, F>(self, other: Option<U>, f: F) -> Option<R>
    where
        F: Fn(T, U) -> R;

    /**
    Combine two Options with the Sum operation.
    ```
    use claudiofsr_lib::OptionExtension;

    let a = Some(5);
    let b = Some(10);

    let result_sum = a.combine_with_sum(b);
    assert_eq!(result_sum, Some(15));
    ```
    */
    fn combine_with_sum<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Add<U, Output = R>;

    /**
    Combine two Options with the Subtraction operation.
    ```
    use claudiofsr_lib::OptionExtension;

    let a = Some(5);
    let b = Some(10);

    let result_sub = a.combine_with_sub(b);
    assert_eq!(result_sub, Some(-5));
    ```
    */
    fn combine_with_sub<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Sub<U, Output = R>;

   /**
    Combine two Options with the Multiplication operation.
    ```
    use claudiofsr_lib::OptionExtension;

    let a = Some(5);
    let b = Some(10);

    let result_mul = a.combine_with_mul(b);
    assert_eq!(result_mul, Some(50));
    ```
    */
    fn combine_with_mul<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Mul<U, Output = R>;

   /**
    Combine two Options with the Division operation.
    ```
    use claudiofsr_lib::OptionExtension;

    let a = Some(50);
    let b = Some(10);

    let result_div = a.combine_with_div(b);
    assert_eq!(result_div, Some(5));
    ```
    */
    fn combine_with_div<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Div<U, Output = R>;

    /**
    Converts `Option<T>` to a String.
    ```
    use claudiofsr_lib::OptionExtension;

    let a: Option<&str> = Some("foo bar");
    let b: Option<u16> = Some(50);
    let c: Option<f64> = Some(10.00);
    let d: Option<f64> = Some(10.700);
    let e: Option<f32> = Some(0.0);
    let f: Option<f32> = Some(0.00000);
    let g: Option<usize> = None;

    let mut strings = Vec::new();

    strings.push(a.to_string());
    strings.push(b.to_string());
    strings.push(c.to_string());
    strings.push(d.to_string());
    strings.push(e.to_string());
    strings.push(f.to_string());
    strings.push(g.to_string());

    assert_eq!(
        strings,
        [
            "foo bar", "50",
            "10.0", "10.7",
            "0.0", "0.0",
            ""
        ]
    );
    ```
    */
    fn to_string(&self) -> String;
}

impl<T> OptionExtension<T> for Option<T>
where
    T: Display
{
    fn combine_with<U, R, F>(self, other: Option<U>, f: F) -> Option<R>
    where
        F: Fn(T, U) -> R,
    {
        self.zip(other).map(|(x, y)| f(x, y))
    }

    fn combine_with_sum<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Add<U, Output = R>
    {
        let sum = |a, b| {a + b};
        self.combine_with(other, sum)
    }

    fn combine_with_sub<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Sub<U, Output = R>
    {
        let sum = |a, b| {a - b};
        self.combine_with(other, sum)
    }

    fn combine_with_mul<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Mul<U, Output = R>
    {
        let mul = |a, b| {a * b};
        self.combine_with(other, mul)
    }

    fn combine_with_div<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Div<U, Output = R>
    {
        let mul = |a, b| {a / b};
        self.combine_with(other, mul)
    }

    fn to_string(&self) -> String {
        /*
        self
            .iter()
            .map(|type_t| {
                type_t.to_string()
            })
            .collect()
        */

        /*
        match self {
            Some(type_t) => type_t.to_string(),
            None => "".to_string(),
        }
        */

        // Avoid converting
        // float "0.0"
        // to
        // integer "0"

        match self {
            Some(type_t) => {
                let type_name = std::any::type_name::<T>();
                //println!("type_name: {type_name}");
                match type_name {
                    "f32" | "f64" => {
                        if let Ok(float_64) = type_t.to_string().parse::<f64>() {
                            // Check if a float can be converted to integer without loss
                            if float_64.trunc() == float_64 {
                                format!("{float_64:.1}")
                            } else {
                                float_64.to_string()
                            }
                        } else {
                            type_t.to_string()
                        }
                    },
                    _ => type_t.to_string(),
                }
            },
            None => "".to_string(),
        }
    }
}
