// https://stackoverflow.com/questions/72888582/choose-thousands-and-decimal-separators-for-f64-in-rust
// https://stackoverflow.com/questions/57029974/how-to-split-string-into-chunks-in-rust-to-insert-spaces
/// Add the thousands separator to float64 with the specified decimal number.
///
/// Example:
/// ```
///     use claudiofsr_lib::thousands_separator;
///
///     let float64: f64 = -2987954368.369177;
///     let decimal: usize = 2;
///     let result: String = thousands_separator(float64, decimal);
///
///     assert_eq!(result, "-2.987.954.368,37");
/// ```
pub fn thousands_separator(value: f64, decimal: usize) -> String {
    let abs_value: f64 = value.abs(); // absolute value
    let round: String = format!("{abs_value:0.decimal$}");

    // integer and fractional part of f64 numbers,
    let integer: &str = &round[..(round.len() - decimal - 1)];
    let fraction: &str = &round[(round.len() - decimal)..];

    let decimal_sep: &str = ",";
    let thousands_sep: char = '.';

    let integer_splitted: String = split_and_insert(integer, thousands_sep);

    if value.is_sign_negative() {
        "-".to_string() + &integer_splitted + decimal_sep + fraction
    } else {
        integer_splitted + decimal_sep + fraction
    }
}

fn split_and_insert(integer: &str, insert: char) -> String {
    let group_size = 3;

    let string_splitted: String = integer
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if (integer.len() - i) % group_size == 0 && i > 0 {
                Some(insert)
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>();

    string_splitted
}

#[allow(dead_code)]
fn split_and_insert_alternative(integer: &str, insert: char) -> String {
    let group_size = 3;

    // Get chars from string.
    let chars: Vec<char> = integer.chars().collect();
    // Allocate new string.
    let mut string_splitted = String::new();
    // Add characters and thousands_sep in sequence
    let mut i = 0;
    loop {
        let j = integer.len() - i;
        if j % group_size == 0 && i > 0 {
            string_splitted.push(insert);
        }
        string_splitted.push(chars[i]);
        //println!("i: {} ; j: {} ; string_splitted: {}", i, j, string_splitted);
        if i >= (integer.len() - 1) {
            break;
        }
        i += 1;
    }

    string_splitted
}

/// Add the thousands separator to float64 with the specified decimal number.
///
/// Example:
/// ```
///     use claudiofsr_lib::thousands_separator_v2;
///
///     let float64: f64 = -2987954368.369177;
///     let decimal: usize = 2;
///     let result: String = thousands_separator_v2(float64, decimal);
///
///     assert_eq!(result, "-2.987.954.368,37");
/// ```
///
/// <https://stackoverflow.com/questions/26998485/is-it-possible-to-print-a-number-formatted-with-thousand-separator-in-rust>
pub fn thousands_separator_v2(value: f64, decimal: usize) -> String {
    let round: String = format!("{value:0.decimal$}");

    let decimal_sep: u8 = b',';
    let thousands_sep: char = '.';
    let group_size: usize = 3;

    // Position of the `.`
    let dot_position: usize = round.bytes().position(|c| c == b'.').unwrap_or(round.len());
    // Is the number negative (starts with `-`)?
    let negative: bool = value.is_sign_negative();
    // Number of integer digits remaning (between the `-` or start and the `.`).
    let mut integer_digits_remaining = dot_position - usize::from(negative);
    // Output. Add capacity for commas. It's a slight over-estimate but that's fine.
    let mut formatted = String::with_capacity(round.len() + integer_digits_remaining / group_size);

    // We can iterate on bytes because everything must be ASCII. Slightly faster.
    for (index, mut byte) in round.bytes().enumerate() {
        match byte {
            b'.' => {
                // Change the decimal separator from '.' for decimal_sep
                byte = decimal_sep;
            }
            b'0'..=b'9' => {
                // Possibly add a thousands_sep.
                if integer_digits_remaining > 0 {
                    // Don't add a thousands_sep at the start of the string.
                    // usize::from(negative); // if negative { 1 } else { 0 }
                    if index != usize::from(negative) && integer_digits_remaining % group_size == 0
                    {
                        formatted.push(thousands_sep);
                    }
                    integer_digits_remaining -= 1;
                }
            }
            _ => (),
        }
        formatted.push(byte as char);
    }

    formatted
}

#[cfg(test)]
mod functions {
    use super::*;
    use std::error::Error;

    // cargo test -- --help
    // cargo test -- --nocapture
    // cargo test -- --show-output

    #[test]
    fn test_thousands_separator() -> Result<(), Box<dyn Error>> {
        // cargo test -- --show-output test_thousands_separator

        let tuples: Vec<(f64, usize)> = vec![
            (-2987954368.369177, 2),
            (123.4, 3),
            (1234.5, 3),
            (1234.5, 1),
            (12345.54321, 8),
            (-0.15, 4),
            (1234566.996, 2),
        ];

        let result: Vec<String> = tuples
            .iter()
            .map(|(value, decimal)| thousands_separator(*value, *decimal))
            .collect();

        let valid = vec![
            "-2.987.954.368,37",
            "123,400",
            "1.234,500",
            "1.234,5",
            "12.345,54321000",
            "-0,1500",
            "1.234.567,00",
        ];

        for ((n, d), r) in tuples.iter().zip(&result) {
            println!("value: {n:20}, decimal: {d} => {r:#?}");
        }

        assert_eq!(valid, result);

        Ok(())
    }
}
