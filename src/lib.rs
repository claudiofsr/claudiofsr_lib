use blake3::Hasher as Blake3Hasher;
use chrono::NaiveDate;
use indicatif::{ProgressBar, ProgressStyle};

use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufReader, Read, Write},
    ops::Deref,
    path::{self, Path},
    process::Command,
    str,
};

mod constants;
mod count_digits;
mod iterations;
mod macros;
mod maps;
mod operations;
mod options;
mod random;
mod rounded;
mod separator;
mod slice;
mod strings;
mod traits;
mod unique;

pub use self::{
    constants::*, count_digits::*, iterations::*, macros::*, maps::*, operations::*, options::*,
    random::*, rounded::*, separator::*, slice::*, strings::*, traits::*, unique::*,
};

pub type MyError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type MyResult<T> = Result<T, MyError>;

const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Clear (wipe) the terminal screen
pub fn clear_terminal_screen() {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).spawn()
    } else {
        // "clear" or "tput reset"
        Command::new("tput").arg("reset").spawn()
    };

    // Alternative solution:
    if result.is_err()
        && let Some(esc) = char::from_digit(27, 10)
    {
        print!("{esc}c");
    }
}

// https://stackoverflow.com/questions/56620265/how-to-access-the-bufreader-twice/

/// Opens a file at the specified path.
///
/// This function attempts to open a file in read-only mode (`true` for `read`) and write-only mode (`false` for `write`).
/// It does not create new files if they do not already exist.
pub fn open_file<P>(path: P) -> Result<File, std::io::Error>
where
    P: AsRef<Path>,
{
    // Create a new OpenOptions instance with the specified read and write modes.
    fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        // Attempt to open the file at the specified path.
        .open(path)
}

/// Byte slice `&[u8]` extension.
pub trait BytesExtension {
    /**
    Trim ascii whitespace from the start and end of `&[u8]`.

    Returns `&[u8]` with leading and trailing whitespace removed.

    Example:
    ```
        use claudiofsr_lib::BytesExtension;

        let text1: &str = " foo bar\r\n";
        let text2: &str = "foo\nbar";
        let text3: &str = "";

        let bytes1: Vec<u8> = text1.bytes().collect();
        let bytes2: Vec<u8> = text2.bytes().collect();
        let bytes3: Vec<u8> = text3.bytes().collect();

        println!("bytes1: {bytes1:?}");
        println!("bytes2: {bytes2:?}");
        println!("bytes3: {bytes3:?}");

        let trimmed1: &[u8] = bytes1.trim();
        let trimmed2: &[u8] = bytes2.trim();
        let trimmed3: &[u8] = bytes3.trim();

        println!("trimmed1: {trimmed1:?}");
        println!("trimmed2: {trimmed2:?}");
        println!("trimmed3: {trimmed3:?}");

        assert_eq!(bytes1, [32, 102, 111, 111, 32, 98, 97, 114, 13, 10]);
        assert_eq!(bytes2, [102, 111, 111, 10, 98, 97, 114]);
        assert!(bytes3.is_empty());

        assert_eq!(trimmed1, [102, 111, 111, 32, 98, 97, 114]);
        assert_eq!(trimmed2, [102, 111, 111, 10, 98, 97, 114]);
        assert_eq!(trimmed3, []);
    ```
    <https://stackoverflow.com/questions/31101915/how-to-implement-trim-for-vecu8>
    */
    fn trim(&self) -> &Self;

    /**
    `&[u8]` to  hex string

    Example:
    ```
        use claudiofsr_lib::BytesExtension;

        let text: &str = " foo bar\n";
        let bytes: Vec<u8> = text.bytes().collect();

        println!("bytes: {bytes:?}");

        let string = bytes.to_hex_string();

        println!("string: {string:?}");

        assert_eq!(bytes, [32, 102, 111, 111, 32, 98, 97, 114, 10]);
        assert_eq!(string, "20666f6f206261720a");
    ```
    */
    fn to_hex_string(&self) -> String;
}

impl BytesExtension for [u8] {
    fn trim(&self) -> &[u8] {
        let from = match self.iter().position(|b| !b.is_ascii_whitespace()) {
            Some(index) => index,
            None => return &self[..0],
        };
        let to = self.iter().rposition(|b| !b.is_ascii_whitespace()).unwrap();
        &self[from..=to]
    }

    fn to_hex_string(&self) -> String {
        self.iter()
            .flat_map(|byte| {
                let a: char = HEX[(*byte as usize) / 16];
                let b: char = HEX[(*byte as usize) % 16];
                vec![a, b]
            })
            .collect()
    }
}

/**
Convert Vec\<&str\> to Vec\<String\>

<https://www.reddit.com/r/learnrust/comments/h82em8/best_way_to_create_a_vecstring_from_str>

Example:
```
    use claudiofsr_lib::to_vec_string;
    let original: Vec<&str> = vec!["this", "that", "the other"];
    let result: Vec<String> = to_vec_string(&original);
    assert_eq!(result, vec![
        String::from("this"),
        String::from("that"),
        String::from("the other"),
    ]);
```
*/
#[allow(dead_code)]
pub fn to_vec_string<T>(v: &[T]) -> Vec<String>
where
    T: ToString,
{
    v.iter().map(|x| x.to_string()).collect()
}

/**
Convert Vec\<String\> to Vec\<&str\>

<https://stackoverflow.com/questions/41179659/convert-vecstring-into-a-slice-of-str-in-rust>

Example:
```
    use claudiofsr_lib::{svec, to_vec_slice};
    let original: Vec<String> = svec!["this", "that", "the other"];
    let result: Vec<&str> = to_vec_slice(&original);
    assert_eq!(result, vec![
        "this",
        "that",
        "the other",
    ]);
```
*/
#[allow(dead_code)]
pub fn to_vec_slice<T>(v: &[T]) -> Vec<&str>
where
    T: AsRef<str>,
{
    v.iter().map(|x| x.as_ref()).collect()
}

/// Gets Date from a string containing 8 digits.
///
/// Date format: DDMMYYYY.
///
/// Check if NaiveDate is valid.
///
/// Returns None on the out-of-range date, invalid month and/or day.
///
/// <https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html#method.from_ymd_opt>
///
/// <https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.parse_from_str>
///
/// Example:
/// ```
///     use claudiofsr_lib::get_naive_date;
///     use chrono::{NaiveDate, Datelike};
///
///     let date_str1: &str = "06-12-2022T00:00:00-03:00";
///     let date1: Option<NaiveDate> = get_naive_date(date_str1);
///     let tuple: Option<(i32, u32, u32)> = date1.map(|dt|(dt.year(), dt.month(), dt.day()));
///
///     let date_str2: &str = "29021973";
///     let date2: Option<NaiveDate> = get_naive_date(date_str2);
///
///     assert_eq!(date1, NaiveDate::from_ymd_opt(2022, 12, 6));
///     assert_eq!(tuple, Some((2022, 12, 6)));
///     assert_eq!(date2, None);
/// ```
pub fn get_naive_date<T>(date: T) -> Option<NaiveDate>
where
    T: Deref<Target = str> + std::fmt::Display,
{
    // "06-12-2022T00:00:00-03:00" -> "061220220000000300"
    let digits: String = date.remove_non_digits();

    let ddmmyyyy: &str = if digits.chars_count() >= 8 {
        &digits[..8]
    } else {
        return None;
    };

    match NaiveDate::parse_from_str(ddmmyyyy, "%-d%-m%Y") {
        Ok(dt) => Some(dt),
        Err(why) => {
            eprintln!("fn get_naive_date()");
            eprintln!("Data inválida ou inexistente!");
            eprintln!("Erro: {why}");
            eprintln!("\t'{date}'");
            None
        }
    }
}

/// Gets Date from a string containing 8 digits.
pub fn get_naive_date_v2<T>(date: T) -> Option<NaiveDate>
where
    T: Deref<Target = str> + std::fmt::Display,
{
    let digits: String = date.remove_non_digits();

    // date: DDMMYYYY
    let ddmmyyyy: u32 = if digits.chars_count() >= 8 {
        digits[..8]
            .parse::<u32>()
            .expect("fn get_naive_date()\nEsperado um número inteiro com 8 dígitos!")
    } else {
        return None;
    };

    let day = ddmmyyyy / 1_000_000;
    let mmyyyy = ddmmyyyy % 1_000_000;

    let month = mmyyyy / 10_000;
    let year = mmyyyy % 10_000;

    let dt: Option<NaiveDate> = NaiveDate::from_ymd_opt(year as i32, month, day);

    if dt.is_none() {
        eprintln!("Erro! Data inválida ou inexistente:");
        eprintln!("\t'{date}': day: {day} ; month: {month} ; year: {year}");
    }

    dt
}

// https://stackoverflow.com/questions/26536871/how-can-i-convert-a-string-of-numbers-to-an-array-or-vector-of-integers-in-rust
/// Convert a string of digits to a vector of digits Vec\<u32\>
///
/// Example:
/// ```
///     use claudiofsr_lib::string_to_vec_of_integers;
///
///     let digits_slice: &str = "06 12 2022";
///     let vector1: Vec<u32> = string_to_vec_of_integers(digits_slice).unwrap();
///
///     let digits_string: String = "06 12 2022".to_string();
///     let vector2: Vec<u32> = string_to_vec_of_integers(digits_string).unwrap();
///
///     assert_eq!(vector1, vector2);
///     assert_eq!(vector1, vec![6, 12, 2022]);
/// ```
pub fn string_to_vec_of_integers<T>(string: T) -> MyResult<Vec<u32>>
where
    T: Deref<Target = str>,
{
    let vec_u32: Result<Vec<u32>, _> = string // Error: ParseIntError
        .split_whitespace()
        .map(|s: &str| s.parse::<u32>())
        .collect();

    Ok(vec_u32?)
}

/// Command line progress with indicatif ProgressBar
pub fn get_progressbar(msg: &'static str, total: usize) -> MyResult<ProgressBar> {
    let style = get_style(0, 0, 38)?;

    let pb = ProgressBar::new(total.try_into()?);
    pb.set_message(msg);
    pb.set_style(style);

    Ok(pb)
}

/// Genarate ProgressStyle by template and progress characters.
pub fn get_style(
    template_index: usize,
    chars_index: usize,
    length: usize,
) -> MyResult<ProgressStyle> {
    let template_01 = format!(
        "{{msg:{length}}} {{spinner:.green}} [{{wide_bar:.cyan/blue}}] {{percent}}/100% ({{eta}}) [{{elapsed_precise}}]"
    );
    let template_02 = format!(
        "{{msg:{length}}} {{spinner:.green}} [{{wide_bar:.cyan/blue}}] {{percent}}/100% ({{eta}})"
    );
    let template_03 = format!(
        "{{msg:{length}}} {{spinner:.green}} [{{wide_bar:.cyan/blue}}] {{pos}}/{{len}} ({{eta}})"
    );
    let template_04 =
        format!("[{{elapsed_precise}}] {{bar:40.cyan/blue}} {{pos}}/{{len}} {{msg:{length}}}");

    let templates = [template_01, template_02, template_03, template_04];

    let progress_characters = ["#>-", "## ", "■□ ", "█░-", "🦀👾👻"];

    let style: ProgressStyle = ProgressStyle::default_bar()
        .template(&templates[template_index])?
        .progress_chars(progress_characters[chars_index]);

    Ok(style)
}

/// Print to file and to stdout
pub fn my_print<P>(write_buffer: &[u8], path: P) -> MyResult<()>
where
    P: AsRef<path::Path>,
{
    // Print to file
    let mut file = fs::File::create(path)?;
    file.write_all(write_buffer)?;

    // Converts a slice of bytes to a string slice
    let print_msg = match str::from_utf8(write_buffer) {
        Ok(valid_uft8) => valid_uft8,
        Err(error) => {
            eprintln!("fn my_print()");
            eprintln!("Invalid UTF-8 sequence!");
            panic!("{error}");
        }
    };

    // Print to stdout
    // writeln!(std::io::stdout(), "{print_msg}")?;
    println!("{print_msg}");

    Ok(())
}

/// Calculates the Blake3 hash from Path.
///
/// <https://docs.rs/blake3/latest/blake3>
///
/// <https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html>
pub fn blake3_hash<P>(path: P) -> MyResult<String>
where
    P: AsRef<Path> + std::marker::Copy + std::fmt::Debug,
{
    let file: File = open_file(path)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut buffer = [0; 1024];

    let mut hasher = Blake3Hasher::new();

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let hash: String = hasher.finalize().to_string();

    Ok(hash)
}

/// Split a slice into smaller slices of size N.
///
/// Then print the result.
pub fn print_split<T>(values: &[T], size: usize)
where
    T: std::fmt::Debug,
{
    for value in values.chunks(size) {
        let text = value
            .iter()
            .map(|v| format!("{v:?}"))
            .collect::<Vec<String>>()
            .join(" ");
        println!("{text}");
    }
}

/// Partition into unique and duplicate slice elements.
pub trait Partition<T> {
    /**
    Partition into unique and duplicate slice elements.

    Order is preserved.

    Example:
    ```
        use claudiofsr_lib::Partition;

        let mut items_a: Vec<u16> = vec![1, 3, 2, 1, 5, 2, 9, 2];
        let (unique, duplicate): (Vec<u16>, Vec<u16>) = items_a.partition_dup();
        assert_eq!(unique, [3, 5, 9]);
        assert_eq!(duplicate, [1, 2, 1, 2, 2]);

        let mut items_b = vec![1, 3, 2, 5, 9];
        let (unique, duplicate) = items_b.partition_dup();
        assert_eq!(unique, [1, 3, 2, 5, 9]);
        assert_eq!(duplicate, []);
    ```
    */
    fn partition_dup(&self) -> (Vec<T>, Vec<T>);
}

impl<T> Partition<T> for Vec<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    fn partition_dup(&self) -> (Vec<T>, Vec<T>) {
        let mut seen: HashSet<T> = HashSet::new();
        let mut filter: HashMap<T, bool> = HashMap::new();
        let mut unique = Vec::new();
        let mut duplicate = Vec::new();

        self.iter().for_each(|item| {
            filter.insert(item.clone(), seen.insert(item.clone()));
        });

        self.iter().for_each(|item| {
            let is_unique: bool = filter[item];
            if is_unique {
                unique.push(item.clone());
            } else {
                duplicate.push(item.clone());
            }
        });

        (unique, duplicate)
    }
}

#[cfg(test)]
mod functions {
    use super::*;
    use chrono::NaiveDate;
    use std::collections::HashMap;

    // cargo test -- --help
    // cargo test -- --nocapture
    // cargo test -- --show-output

    #[test]
    /// `cargo test -- --show-output clear_terminal`
    fn clear_terminal() {
        clear_terminal_screen()
    }

    #[test]
    fn partition_values() {
        // cargo test -- --show-output partition_values

        let vector: Vec<String> = ["aa", "ab", "aa", "aa", "cc", "aa", "ab", "d1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        println!("vector: {vector:?}");

        let (u, d) = vector.partition_dup();
        println!("unique: {u:?}");
        println!("duplicate: {d:?}");

        assert_eq!(u, ["cc", "d1"]);
        assert_eq!(d, ["aa", "ab", "aa", "aa", "aa", "ab"]);
    }

    #[test]
    fn basic_trimming() {
        // https://stackoverflow.com/questions/31101915/how-to-implement-trim-for-vecu8
        // cargo test -- basic_trimming
        assert_eq!(b" A ".trim(), b"A");
        assert_eq!(b" AB ".trim(), b"AB");
        assert_eq!(b"A ".trim(), b"A");
        assert_eq!(b"AB ".trim(), b"AB");
        assert_eq!(b" A".trim(), b"A");
        assert_eq!(b" AB".trim(), b"AB");
        assert_eq!(b" A B ".trim(), b"A B");
        assert_eq!(b"A B ".trim(), b"A B");
        assert_eq!(b" A B".trim(), b"A B");
        assert_eq!(b" ".trim(), b"");
        assert_eq!(b"  ".trim(), b"");
        assert_eq!(b"\nA\n".trim(), b"A");
        assert_eq!(b"\nA  B\r\n".trim(), b"A  B");
        assert_eq!(b"\r\nA  B\r\n".trim(), b"A  B");
    }

    #[test]
    fn data_dia_mes_ano() -> MyResult<()> {
        // cargo test -- --show-output data_dia_mes_ano

        for (date, result) in [
            ("18052022", NaiveDate::from_ymd_opt(2022, 5, 18)),
            ("15121500", NaiveDate::from_ymd_opt(1500, 12, 15)),
            ("29021972", NaiveDate::from_ymd_opt(1972, 2, 29)),
            ("18152022", None),
            ("29021973", None),
            ("2921972", None),
            ("2023", None),
            ("", None),
        ] {
            println!("date: '{date}' ; result: {result:?}");
            let naive_date: Option<NaiveDate> = get_naive_date(date);
            assert_eq!(naive_date, result);
        }

        Ok(())
    }

    #[test]
    fn test_group_anagrams() -> MyResult<()> {
        // cargo test -- --show-output test_group_anagrams
        // https://leetcode.com/problems/group-anagrams/description/
        // https://leetcode.com/problems/group-anagrams/solutions/2155441/rust-hashmap-solution-simple/

        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut h = HashMap::new();

            for s in strs {
                let mut key: Vec<char> = s.chars().collect();
                key.sort();
                h.entry(key).or_insert(Vec::new()).push(s);
            }

            h.into_values().collect()
        }

        let text = svec!("abc", "bac", "def", "ab", "tuvxz", "abc", "a", "bca", "fed");
        let result = group_anagrams(text);

        println!("result: {result:?}");

        Ok(())
    }
}
