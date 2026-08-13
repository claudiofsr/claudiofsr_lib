use std::{fmt::Display, ops::Deref};

/// Adds utility methods to the `Option<T>` type.
pub trait OptionExtension<T> {
    /// Combines two Options into one using a custom closure.
    ///
    /// # Examples
    ///
    /// ```
    /// use claudiofsr_lib::OptionExtension;
    ///
    /// let a = Some(5.0);
    /// let b = Some(10.0);
    ///
    /// let sum = |a, b| { a + b };
    /// let sub = |a, b| { a - b };
    /// let mul = |a, b| { a * b };
    /// let div = |a, b| { a / b };
    ///
    /// assert_eq!(a.combine_with(b, sum), Some(15.0));
    /// assert_eq!(a.combine_with(b, sub), Some(-5.0));
    /// assert_eq!(a.combine_with(b, mul), Some(50.0));
    /// assert_eq!(a.combine_with(b, div), Some(0.5));
    /// ```
    fn combine_with<U, R, F>(self, other: Option<U>, f: F) -> Option<R>
    where
        F: Fn(T, U) -> R;

    /// Combines two Options using the Addition (`+`) operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use claudiofsr_lib::OptionExtension;
    ///
    /// let a = Some(5);
    /// let b = Some(10);
    ///
    /// assert_eq!(a.combine_with_sum(b), Some(15));
    /// ```
    fn combine_with_sum<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Add<U, Output = R>;

    /// Combines two Options using the Subtraction (`-`) operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use claudiofsr_lib::OptionExtension;
    ///
    /// let a = Some(5);
    /// let b = Some(10);
    ///
    /// assert_eq!(a.combine_with_sub(b), Some(-5));
    /// ```
    fn combine_with_sub<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Sub<U, Output = R>;

    /// Combines two Options using the Multiplication (`*`) operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use claudiofsr_lib::OptionExtension;
    ///
    /// let a = Some(5);
    /// let b = Some(10);
    ///
    /// assert_eq!(a.combine_with_mul(b), Some(50));
    /// ```
    fn combine_with_mul<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Mul<U, Output = R>;

    /// Combines two Options using the Division (`/`) operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use claudiofsr_lib::OptionExtension;
    ///
    /// let a = Some(50);
    /// let b = Some(10);
    ///
    /// assert_eq!(a.combine_with_div(b), Some(5));
    /// ```
    fn combine_with_div<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Div<U, Output = R>;

    /// Converts `Option<T>` to a `String`, with safe float truncation prevention.
    ///
    /// # Examples
    ///
    /// ```
    /// use claudiofsr_lib::OptionExtension;
    ///
    /// let a: Option<&str> = Some("foo bar");
    /// let b: Option<u16> = Some(50);
    /// let c: Option<f64> = Some(10.00);
    /// let d: Option<f64> = Some(10.700);
    /// let e: Option<f32> = Some(0.0);
    /// let f: Option<f32> = Some(0.00000);
    /// let g: Option<usize> = None;
    ///
    /// assert_eq!(a.to_string(), "foo bar");
    /// assert_eq!(b.to_string(), "50");
    /// assert_eq!(c.to_string(), "10.0");
    /// assert_eq!(d.to_string(), "10.7");
    /// assert_eq!(e.to_string(), "0.0");
    /// assert_eq!(f.to_string(), "0.0");
    /// assert_eq!(g.to_string(), "");
    /// ```
    fn to_string(&self) -> String;

    /// Parses an `Option<T>` into an `Option<U>` after stripping whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use claudiofsr_lib::OptionExtension;
    ///
    /// let a: Option<u64> = Some(56);
    /// let b: Option<&str> = Some(" 56 ");
    /// let c: Option<&str> = Some(" foo bar \n");
    /// let d: Option<String> = Some("379.32000".to_string());
    /// let e: Option<String> = Some("379.32   ".to_string());
    ///
    /// let result_a: Option<u8> = a.parse_opt();
    /// let result_b: Option<u8> = b.parse_opt();
    /// let result_c: Option<String> = c.parse_opt();
    /// let result_d: Option<f32> = d.parse_opt();
    /// let result_e: Option<f64> = e.parse_opt();
    /// let result_f: Option<u64> = e.parse_opt();
    ///
    /// assert_eq!(result_a, Some(56));
    /// assert_eq!(result_b, Some(56));
    /// assert_eq!(result_c, Some("foo bar".to_string()));
    /// assert_eq!(result_d, Some(379.32));
    /// assert_eq!(result_e, Some(379.32));
    /// assert_eq!(result_f, None);
    /// ```
    fn parse_opt<U>(&self) -> Option<U>
    where
        U: std::str::FromStr;

    /// Retains only ASCII digit characters from an `Option<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use claudiofsr_lib::OptionExtension;
    ///
    /// let opt_none: Option<&str> = None;
    /// assert_eq!(opt_none.retain_only_digits(), None);
    ///
    /// let opt_mixed: Option<&str> = Some("abc123-def;!456 @");
    /// assert_eq!(opt_mixed.retain_only_digits(), Some("123456".to_string()));
    ///
    /// let opt_digits: Option<String> = Some("0123456789".to_string());
    /// assert_eq!(opt_digits.retain_only_digits(), Some("0123456789".to_string()));
    ///
    /// let opt_letters: Option<&str> = Some("abcdefg");
    /// assert_eq!(opt_letters.retain_only_digits(), None);
    /// ```
    fn retain_only_digits(&self) -> Option<String>
    where
        T: Deref<Target = str>;
}

impl<T> OptionExtension<T> for Option<T>
where
    T: Display,
{
    #[inline]
    fn combine_with<U, R, F>(self, other: Option<U>, f: F) -> Option<R>
    where
        F: Fn(T, U) -> R,
    {
        // Zips self with another Option.
        // If self is Some(x) and other is Some(y), this method returns Some((x, y)).
        // Otherwise, None is returned.
        self.zip(other).map(|(x, y)| f(x, y))
    }

    #[inline]
    fn combine_with_sum<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Add<U, Output = R>,
    {
        self.combine_with(other, |a, b| a + b)
    }

    #[inline]
    fn combine_with_sub<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Sub<U, Output = R>,
    {
        self.combine_with(other, |a, b| a - b)
    }

    #[inline]
    fn combine_with_mul<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Mul<U, Output = R>,
    {
        self.combine_with(other, |a, b| a * b)
    }

    #[inline]
    fn combine_with_div<U, R>(self, other: Option<U>) -> Option<R>
    where
        T: std::ops::Div<U, Output = R>,
    {
        self.combine_with(other, |a, b| a / b)
    }

    fn to_string(&self) -> String {
        match self {
            Some(type_t) => {
                let type_name = std::any::type_name::<T>();
                if (type_name == "f32" || type_name == "f64")
                    && let Ok(float_64) = type_t.to_string().parse::<f64>()
                    && float_64.trunc() == float_64
                {
                    return format!("{float_64:.1}");
                }
                type_t.to_string()
            }
            None => String::new(),
        }
    }

    #[inline]
    fn parse_opt<U>(&self) -> Option<U>
    where
        U: std::str::FromStr,
    {
        self.as_ref()
            .and_then(|entry| entry.to_string().trim().parse::<U>().ok())
    }

    #[inline]
    fn retain_only_digits(&self) -> Option<String>
    where
        T: Deref<Target = str>,
    {
        let text = self.as_deref()?;
        let digits: String = text.chars().filter(char::is_ascii_digit).collect();
        (!digits.is_empty()).then_some(digits)
    }
}

#[cfg(test)]
mod options_tests {
    use super::*;

    /// `cargo test -- --show-output retain_only_digits`
    #[test]
    fn retain_only_digits_empty_string() {
        let opt_str: Option<&str> = None;
        assert_eq!(opt_str.retain_only_digits(), None);
    }

    #[test]
    fn retain_only_digits_non_digit_characters() {
        let opt_str: Option<&str> = Some("abc123-def;!456@ ");
        assert_eq!(opt_str.retain_only_digits(), Some("123456".to_string()));
    }

    #[test]
    fn retain_only_digits_all_digits() {
        let opt_str: Option<&str> = Some("0123456789");
        assert_eq!(opt_str.retain_only_digits(), Some("0123456789".to_string()));
    }

    #[test]
    fn retain_only_digits_no_digits() {
        let opt_str: Option<&str> = Some("abcdefg");
        assert_eq!(opt_str.retain_only_digits(), None);
    }
}
