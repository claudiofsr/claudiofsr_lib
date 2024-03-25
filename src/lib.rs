use indicatif::{ProgressBar, ProgressStyle};
use blake3::Hasher as Blake3Hasher;
use chrono::NaiveDate;

use std::{
    str,
    ops::Deref,
    error::Error,
    process::Command,
    fs::{self, File},
    path::{self, Path},
    collections::{HashSet, HashMap},
    io::{Write, Read, BufRead, BufReader},
};

mod constants;
mod macros;
mod options;
mod separator;
mod slice;
mod strings;

pub use self::{
    constants::*,
    macros::*,
    options::*,
    separator::*,
    slice::*,
    strings::*,
};

pub type MyError = Box<dyn std::error::Error + Send + Sync>;
pub type MyResult<T> = Result<T, MyError>;

const HEX: [char; 16] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
	'a', 'b', 'c', 'd', 'e', 'f',
];

// https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
// https://stackoverflow.com/questions/65497187/cant-run-a-system-command-in-windows
/// Clear the terminal screen
pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

// https://stackoverflow.com/questions/69297477/getting-the-length-of-an-int
// https://users.rust-lang.org/t/whats-the-quickest-way-to-get-the-length-of-an-integer
// https://internals.rust-lang.org/t/pre-rfc-lo-and-hi-methods-for-splitting-integers-into-their-halves
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f29cbb40ffce2498c005d390b22dd51c
// https://github.com/blueglyph/ilog
/// Returns the number of digits in an integer.
///
/// The input can be an integer from u8 to u128.
pub fn num_digits<I>(integer: I) -> usize
where
    I: ilog::IntLog
{
    integer.checked_log10().unwrap_or(0) + 1
}

// https://stackoverflow.com/questions/56620265/how-to-access-the-bufreader-twice/
/// File is an object providing access to an open file on the filesystem.
/// Use the seek or rewind functions to reset the position of the files to start.
pub fn open_file<P>(path: P) -> Result<File, Box<dyn Error>>
where
    P: AsRef<Path> + std::marker::Copy + std::fmt::Debug,
{
    let file: File = match fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(path)
        {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Failed to open file {path:?}");
                panic!("{error}");
            }
        };

    Ok(file)
}

/// Adds a counter for the number of lines in a file.
pub trait FileExtension {
    fn count_lines(&mut self) -> Result<u64, Box<dyn Error>>;    // use BufReader
    // fn count_lines(&mut self) -> Result<u64, Box<dyn Error>>; // use memmap2
}

impl FileExtension for File {
    /**
    Count the number of lines in the file.

    Example:
    ```
    use claudiofsr_lib::{FileExtension, open_file};
    use std::{fs::File, io::Write, path::Path, error::Error};

    fn main() -> Result<(), Box<dyn Error>> {

        let filename = "/tmp/sample.txt";
        let mut file = File::create(filename)?;
        file.write_all(b"A test\nActual content\nMore content\nAnother test")?;

        let path = Path::new(filename);
        let mut file: File = open_file(path)?;
        let number_of_lines: u64 = file.count_lines()?;

        assert_eq!(number_of_lines, 4);
        Ok(())
    }
    ````
    */
    fn count_lines(&mut self) -> Result<u64, Box<dyn Error>> {

        let count: u64 = BufReader::new(self)
            //.lines()     // Return an error if the read bytes are not valid UTF-8
            .split(b'\n')  // Ignores invalid UTF-8 but
            .try_count()?; // Catches other errors

        Ok(count)
    }

    /*
    /// Count the number of lines in the file
    ///
    /// use memmap2::Mmap;
    fn count_lines(&mut self) -> Result<u64, Box<dyn Error>> {

        // https://docs.rs/memmap2/latest/memmap2/struct.Mmap.html
        let count: u64 = unsafe { Mmap::map(&*self)? }
            .par_split(|&byte| byte == b'\n') // ignore invalid UTF-8
            .count()
            .try_into()?;

        Ok(count)
    }
    */
}

/**
Count function consumes the Lines:

`let number_of_lines = BufReader::new(file).lines().count();`

`count()` essentially loops over all the elements of the iterator, incrementing a counter until the iterator returns None.

In this case, once the file can no longer be read, the iterator never returns None, and the function runs forever.

You can make the iterator return None when the inner value is an error by using functions such as try_fold().

```
    use claudiofsr_lib::IteratorExtension;
    use std::io::{BufRead, BufReader};

    let text: &str = "this\nis\na\ntest\n";
    let counter: Result<u64, _> = BufReader::new(text.as_bytes())
        //.lines()    // Return an error if the read bytes are not valid UTF-8
        .split(b'\n') // Ignores invalid UTF-8 but
        .try_count(); // Catches other errors

    //assert!(result.is_ok_and(|c| c == 4));
    assert!(matches!(counter, Ok(4)));
```

<https://www.reddit.com/r/rust/comments/wyk1l0/can_you_compose_stdiolines_with_stditercount/>
*/
pub trait IteratorExtension<E> {
    fn try_count(&mut self) -> Result<u64, E>;
}

impl<T, U, E> IteratorExtension<E> for T
where
    T: Iterator<Item = Result<U, E>>
{
    /**
    Try to count the iter number
    ```
        use claudiofsr_lib::IteratorExtension;
        use std::io::{BufRead, BufReader};

        let invalid_unicode = b"\xc3\x28\x30\x0a\x31\x0a\x32\x0a";
        let counter = BufReader::new(&invalid_unicode[..])
                //.lines(); // Return an error if the read bytes are not valid UTF-8
                .split(b'\n')
                .try_count();

        assert!(matches!(counter, Ok(3)));
    ```
    <https://www.reddit.com/r/rust/comments/wyk1l0/can_you_compose_stdiolines_with_stditercount/>
    */
    fn try_count(&mut self) -> Result<u64, E> {
        self.try_fold(0, |accumulator: u64, element: Result<U, E>|
            element.map(|_| accumulator + 1)
        )
    }
}

/// Byte slice `&[u8]` extension.
pub trait BytesExtension {
    fn trim(&self) -> &Self;
    fn to_hex_string(&self) -> String;
}

impl BytesExtension for [u8] {
    /**
    Trim ascii whitespace from the start and end of `&[u8]`.

    Returns `&[u8]` with leading and trailing whitespace removed.

    Example:
    ```
        use claudiofsr_lib::BytesExtension;

        let text: &str = " foo bar\r\n";
        let bytes: Vec<u8> = text.bytes().collect();

        println!("bytes: {bytes:?}");

        let trimmed = bytes.trim();

        println!("trimmed: {trimmed:?}");

        assert_eq!(bytes, [32, 102, 111, 111, 32, 98, 97, 114, 13, 10]);
        assert_eq!(trimmed, [102, 111, 111, 32, 98, 97, 114]);
    ```

    <https://stackoverflow.com/questions/31101915/how-to-implement-trim-for-vecu8>
    */
    fn trim(&self) -> &[u8] {
        let from = match self.iter().position(|x| !x.is_ascii_whitespace()) {
            Some(i) => i,
            None => return &self[0..0],
        };
        let to = self.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
        &self[from..=to]
    }

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
    fn to_hex_string(&self) -> String {
        self.iter()
            .flat_map(|byte| {
                let a: char = HEX[(*byte as usize)/16];
                let b: char = HEX[(*byte as usize)%16];
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
        T: ToString
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
        T: AsRef<str>
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
        T: Deref<Target=str> + std::fmt::Display,
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
        T: Deref<Target=str> + std::fmt::Display,
{
    let digits: String = date.remove_non_digits();

    // date: DDMMYYYY
    let ddmmyyyy: u32 = if digits.chars_count() >= 8 {
        digits[..8].parse::<u32>()
        .expect("fn get_naive_date()\nEsperado um número inteiro com 8 dígitos!")
    } else {
        return None;
    };

    let day    = ddmmyyyy / 1_000_000;
    let mmyyyy = ddmmyyyy % 1_000_000;

    let month = mmyyyy / 10_000;
    let year  = mmyyyy % 10_000;

    let dt: Option<NaiveDate> = NaiveDate::from_ymd_opt(year as i32, month, day);

    if dt.is_none() {
        eprintln!("Erro! Data inválida ou inexistente:");
        eprintln!("\t'{date}': day: {day} ; month: {month} ; year: {year}");
    }

    dt
}

// https://stackoverflow.com/questions/43516351/how-to-convert-a-string-of-digits-into-a-vector-of-digits
// https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait
/// Convert a string of digits to a vector of digits Vec\<u32\>
///
/// Example:
/// ```
///     use claudiofsr_lib::string_to_digits;
///
///     let digits_slice: &str = "06122022";
///     let vector1: Vec<u32> = string_to_digits(digits_slice);
///
///     let digits_string: String = "06122022".to_string();
///     let vector2: Vec<u32> = string_to_digits(digits_string);
///
///     assert_eq!(vector1, vector2);
///     assert_eq!(vector1, vec![0, 6, 1, 2, 2, 0, 2, 2]);
/// ```
pub fn string_to_digits<T>(string: T) -> Vec<u32>
where
    T: Deref<Target=str>,
{
    let opt_vec: Option<Vec<u32>> = string
        .chars()
        .map(|ch: char| ch.to_digit(10))
        .collect();

    match opt_vec {
        Some(vec_of_digits) => vec_of_digits,
        None                => vec![],
    }
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
pub fn string_to_vec_of_integers<T>(string: T) -> Result<Vec<u32>, Box<dyn Error>>
where
    T: Deref<Target=str>,
{
    let vec_u32: Result<Vec<u32>, _> = string // Error: ParseIntError
        .split_whitespace()
        .map(|s: &str| s.parse::<u32>())
        .collect();

    Ok(vec_u32?)
}

/// Two Rounding method for floating-point operations:
///
/// 1. Round to nearest value, ties to even:
///
///     if the number falls midway, it is rounded to the nearest value with an even least significant digit.
///
/// 2. Round to nearest value, ties away from zero (or ties to away):
///
///     if the number falls midway, it is rounded to the nearest value above (for positive numbers) or below (for negative numbers).
///
/// Python takes the first approach and Rust takes the second.
///
/// Neither is contradicting the IEEE-754 standard, which defines and allows for both.
///
/// ```
///     use claudiofsr_lib::round_f64;
///     let decimals: u32 = 2;
///
///     let number01: f64 = 1.454999;
///     let result01: f64 = round_f64(number01, decimals);
///     let expected01: f64 = 1.45;
///     assert!(matches!(expected01, result01));
///
///     let number02: f64 = 1.455000;
///     let result02: f64 = round_f64(number02, decimals);
///     let expected02: f64 = 1.46;
///     assert!(matches!(expected02, result02));
/// ```
/// <https://floating-point-gui.de/languages/rust>
///
/// <https://doc.rust-lang.org/std/primitive.f64.html#method.powf>
///
/// <https://pola-rs.github.io/polars/src/polars_core/series/ops/round.rs.html#8>
pub fn round_f64(value: f64, decimals: u32) -> f64 {
    if decimals == 0 {
        value.round()
    } else {
        let multiplier = 10.0_f64.powf(decimals as f64);
        (value * multiplier).round() / multiplier
    }
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

    let template_01 = format!("{{msg:{}}} {{spinner:.green}} [{{wide_bar:.cyan/blue}}] {{percent}}/100% ({{eta}}) [{{elapsed_precise}}]", length);
    let template_02 = format!("{{msg:{}}} {{spinner:.green}} [{{wide_bar:.cyan/blue}}] {{percent}}/100% ({{eta}})", length);
    let template_03 = format!("{{msg:{}}} {{spinner:.green}} [{{wide_bar:.cyan/blue}}] {{pos}}/{{len}} ({{eta}})", length);
    let template_04 = format!("[{{elapsed_precise}}] {{bar:40.cyan/blue}} {{pos}}/{{len}} {{msg:{}}}", length);

    let templates = [
        template_01,
        template_02,
        template_03,
        template_04,
    ];

    let progress_characters = ["#>-",
        "## ",
        "■□ ",
        "█░-",
        "🦀👾👻",
    ];

    let style: ProgressStyle = ProgressStyle::default_bar()
        .template(&templates[template_index])?
        .progress_chars(progress_characters[chars_index]);

    Ok(style)
}

/// Print to file and to stdout
pub fn my_print<P>(write_buffer: &[u8], path: P) -> Result<(), Box<dyn Error>>
where P: AsRef<path::Path>
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
pub fn blake3_hash<P>(path: P) -> Result<String, Box<dyn Error>>
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
    T: std::fmt::Debug
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

/// Get unique values from vector items.
pub trait Unique<T> {
    /**
    Get unique values from vector items.

    This method operates in place, visiting each element exactly once in the
    original order, and preserves the order of the retained elements.

    Example:
    ```
        use claudiofsr_lib::Unique;

        let mut items = vec![1, 3, 1, 2, 2];
        items.unique();
        assert_eq!(items, vec![1, 3, 2]);
    ```
    This works because retain only keeps items for which the predicate returns true,
    and insert only returns true if the item was not previously present in the set.

    Since the vector is traversed in order, we end up keeping just the first occurrence of each item.

    <https://stackoverflow.com/questions/64819025/is-there-a-simple-way-remove-duplicate-elements-from-an-array>
    */
    fn unique(&mut self);
}

impl<T> Unique<T> for Vec<T>
where
    T: Clone + Eq + std::hash::Hash
{
    fn unique(self: &mut Vec<T>) {
        let mut seen: HashSet<T> = HashSet::new();
        self.retain(|item| seen.insert(item.clone()));
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
    fn unique_values() {
        // cargo test -- --show-output unique_values

        let mut vector = vec![1, 4, 3, 4, 5, 4, 3, 4, 2, 3];
        println!("vector: {:?}", vector);

        vector.unique();
        println!("vector: {:?}", vector);

        assert_eq!(vector, [1, 4, 3, 5, 2]);
    }

    #[test]
    fn partition_values() {
        // cargo test -- --show-output partition_values

        let vector: Vec<String> = ["aa", "ab", "aa", "aa", "cc", "aa", "ab", "d1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        println!("vector: {:?}", vector);

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
    fn data_dia_mes_ano() -> Result<(), Box<dyn Error>> {
        // cargo test -- --show-output data_dia_mes_ano

        for (date, result) in [
            ("18052022", NaiveDate::from_ymd_opt(2022,  5, 18)),
            ("15121500", NaiveDate::from_ymd_opt(1500, 12, 15)),
            ("29021972", NaiveDate::from_ymd_opt(1972,  2, 29)),
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
    fn test_num_digits() -> Result<(), Box<dyn Error>> {
        // cargo test -- --show-output num_digits

        let input: u8 = 0;
        let lenght_u8_zero = num_digits(input);
        println!("num_digits({input}) => {lenght_u8_zero}");

        let input: u8 = 255;
        let lenght_u8 = num_digits(input);
        println!("num_digits({input}) => {lenght_u8}");

        let input: u16 = 65535;
        let lenght_u16 = num_digits(input);
        println!("num_digits({input}) => {lenght_u16}");

        let input: u32 = 4294967295;
        let lenght_u32 = num_digits(input);
        println!("num_digits({input}) => {lenght_u32}");

        let input: f64 = 123456.789;
        let lenght_f64 = num_digits(input as usize);
        println!("num_digits({input}) => {lenght_f64}");

        let input: u64 = 18446744073709551615;
        let lenght_u64 = num_digits(input);
        println!("num_digits({input}) => {lenght_u64}");

        let input: u128 = 340282366920938463463374607431768211455;
        let lenght_u128 = num_digits(input);
        println!("num_digits({input}) => {lenght_u128}");

        assert_eq!(
            (
                lenght_u8_zero, lenght_u8, lenght_u16, lenght_u32,
                lenght_f64, lenght_u64, lenght_u128
            ),
            (1, 3, 5, 10, 6, 20, 39)
        );

        Ok(())
    }

    #[test]
    fn test_group_anagrams() -> Result<(), Box<dyn Error>> {
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
