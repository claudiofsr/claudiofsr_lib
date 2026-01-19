use std::ops::Deref;

/// Trait extension for String
pub trait StringExtension {
    /**
    Remove all whitespace from a string.
    ```
        use claudiofsr_lib::StringExtension;

        let mut string = String::from(" for  bar \n");
        string.remove_all_whitespace();

        assert_eq!(string, "forbar");
    ```
    */
    fn remove_all_whitespace(&mut self);

    /**
    Remove all char from a string.
    ```
        use claudiofsr_lib::StringExtension;

        let mut string = String::from("for bar bbar");
        string.remove_all_char('b');

        assert_eq!(string, "for ar ar");
    ```
    */
    fn remove_all_char(&mut self, c: char);
}

impl StringExtension for String {
    fn remove_all_whitespace(&mut self) {
        self.retain(|c| !c.is_whitespace());
    }

    fn remove_all_char(&mut self, ch: char) {
        self.retain(|c| c != ch);
    }
}

/// Extension trait providing string manipulation utilities.
pub trait StrExtension {
    /**
    Returns the characters count.

    Not use len()
    ```
        use claudiofsr_lib::StrExtension;
        let text_a: &str = "12x45";
        let text_b: &str = "Bom dia おはよう!";
        let text_c: String = " Cláudio 🦀 çṕ@".to_string();
        assert_eq!(text_a.chars_count(), 5);
        assert_eq!(text_b.chars_count(), 13);
        assert_eq!(text_c.chars_count(), 14);
    ```
    */
    fn chars_count(&self) -> usize;

    /**
    Counts the number of occurrences of a given character in a String.
    ```
        use claudiofsr_lib::StrExtension;

        let line1: &str = "|C170|zfoo|bar|zzz|";
        let line2: String = "|C170|zfoo|bar|zzz|".to_string();
        let result1: usize = line1.count_char('|');
        let result2: usize = line2.count_char('z');
        assert_eq!(result1, 5);
        assert_eq!(result2, 4);
    ```
    */
    fn count_char(&self, ch: char) -> usize;

    /**
    Returns true if it has only ASCII decimal digits.
    ```
        use claudiofsr_lib::StrExtension;
        let text_a: &str = "12345";
        let text_b: &str = "12x45";
        assert!(text_a.contains_only_digits());
        assert!(!text_b.contains_only_digits());
    ```
    */
    fn contains_only_digits(&self) -> bool;

    /**
    Returns true if it has some ASCII decimal digits.
    ```
        use claudiofsr_lib::StrExtension;
        let text_a: &str = "12345";
        let text_b: &str = "12x45";
        let text_c: &str = "foo";
        assert!(text_a.contains_some_digits());
        assert!(text_b.contains_some_digits());
        assert!(!text_c.contains_some_digits());
    ```
    */
    fn contains_some_digits(&self) -> bool;

    /**
    Returns true if it has N number of characters and
    all characters are ASCII decimal digits.
    ```
        use claudiofsr_lib::StrExtension;
        let text_a: &str = "12345";
        let text_b: &str = "12x45";
        let text_c: &str = "foo";
        assert!(text_a.contains_num_digits(5));
        assert!(!text_b.contains_num_digits(4));
        assert!(!text_c.contains_num_digits(3));
    ```
    */
    fn contains_num_digits(&self, num_digit: usize) -> bool;

    /**
    Returns true if it has up to N number of characters
    and all characters are ASCII decimal digits.
    ```
        use claudiofsr_lib::StrExtension;
        let text_a: &str = "12345";
        let text_b: &str = "12x45";
        let text_c: &str = "foo";
        assert!(text_a.contains_up_to_num_digits(6));
        assert!(text_a.contains_up_to_num_digits(5));
        assert!(!text_a.contains_up_to_num_digits(4));
        assert!(!text_b.contains_up_to_num_digits(4));
        assert!(!text_c.contains_up_to_num_digits(3));
    ```
    */
    fn contains_up_to_num_digits(&self, num_digit: usize) -> bool;

    /**
    Returns true if all characters are ASCII (0-9a-zA-Z) alphanumeric.
    ```
        use claudiofsr_lib::StrExtension;
        let text_a: &str = "123aB";
        let text_b: &str = "12@45";
        let text_c: &str = "124藏5";
        assert!(text_a.is_ascii_alphanumeric());
        assert!(!text_b.is_ascii_alphanumeric());
        assert!(!text_c.is_ascii_alphanumeric());
    ```
    */
    fn is_ascii_alphanumeric(&self) -> bool;

    /**
    Returns true if all characters are alphanumeric.
    ```
        use claudiofsr_lib::StrExtension;
        let text_a: &str = "123aB";
        let text_b: &str = "12¾45①";
        let text_c: &str = "124藏5";
        let text_d: &str = "124,5";
        assert!(text_a.is_alphanumeric());
        assert!(text_b.is_alphanumeric());
        assert!(text_c.is_alphanumeric());
        assert!(!text_d.is_alphanumeric());
    ```
    */
    fn is_alphanumeric(&self) -> bool;

    /// Replaces multiple consecutive space characters with a single space.
    ///
    /// This method collapses sequences of two or more spaces into one,
    /// while preserving single spaces and the integrity of multi-byte
    /// UTF-8 characters (like accents and emojis).
    ///
    /// ### Performance
    /// - **Fast Path**: Uses SIMD-accelerated detection to return early if no multiple spaces exist.
    /// - **Zero-Reallocation**: Pre-allocates exact capacity to ensure a single heap allocation.
    /// - **Slice-Pushing**: Copies data in blocks using `memcpy` rather than character-by-character.
    ///
    /// ### Examples
    /// ```
    /// use claudiofsr_lib::StrExtension;
    ///
    /// let text_a = "a  bç d";
    /// let text_b = "a   bc    d";
    /// let text_c = "  a    bc d  ";
    ///
    /// assert_eq!("a bç d", text_a.replace_multiple_whitespaces());
    /// assert_eq!("a bc d", text_b.replace_multiple_whitespaces());
    /// assert_eq!(" a bc d ", text_c.replace_multiple_whitespaces());
    /// ```
    fn replace_multiple_whitespaces(&self) -> String;

    /**
    Remove all non-digits characters

    Create string t from string s, keeping only digit characters 0, 1, 2, 3, 4, 5, 6, 7, 8, 9.

    ```
        use claudiofsr_lib::StrExtension;
        let text: &str = "1234-ab_5ção67__8 9 ";
        let result: String = text.remove_non_digits();
        assert_eq!(result, "123456789");
    ```
    */
    fn remove_non_digits(&self) -> String;

    /**
    Remove the first and last character from a string
    ```
        use claudiofsr_lib::StrExtension;
        let text: &str = "1234-ab_5ç";
        let result: String = text.remove_first_and_last_char();
        assert_eq!(result, "234-ab_5");
    ```
    <https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust>
    */
    fn remove_first_and_last_char(&self) -> String;

    /**
    Capture or Retain only the first group of digits:
    ```
        use claudiofsr_lib::StrExtension;

        let text01: &str = "1191-1";
        let result: String = text01.select_first_digits();
        assert_eq!(result, "1191");

        let text02: &str = "10845/a";
        let result: String = text02.select_first_digits();
        assert_eq!(result, "10845");
    ```
    */
    fn select_first_digits(&self) -> String;

    /**
    Retain the first digits
    ```
        use claudiofsr_lib::StrExtension;
        let word = "12345abc678";
        let digits = word.retain_first_digits();

        assert_eq!(digits, "12345");
    ```
    */
    fn retain_first_digits(&self) -> &str;

    /**
    Returns a string with the prefix and suffix delimiter removed.
    ```
        use claudiofsr_lib::StrExtension;
        let text: &str = "12|34|ab|5|ç678";
        let result: &str = text.strip_prefix_and_sufix(b'|');
        assert_eq!(result, "34|ab|5");
    ```
    <https://doc.rust-lang.org/src/core/str/mod.rs.html>
    */
    fn strip_prefix_and_sufix(&self, delimiter_byte: u8) -> &str;

    /**
    Get the first n character of a String or &str.
    ```
        use claudiofsr_lib::StrExtension;

        let string: String = String::from("♥foo よção♥ bar");
        assert_eq!(string.get_first_n_chars(10), "♥foo よção♥");
        assert_eq!(string.get_first_n_chars(14), string);

        let str: &str = "♥foo よção♥ bar";
        assert_eq!(str.get_first_n_chars(10), "♥foo よção♥");
        assert_eq!(str.get_first_n_chars(14), str);
    ```
    */
    fn get_first_n_chars(&self, num: usize) -> &str;

    /**
    Get the last n character of a String or &str.
    ```
        use claudiofsr_lib::StrExtension;

        let string: String = String::from("♥foo よção♥ bar");
        assert_eq!(string.get_last_n_chars(9), "よção♥ bar");
        assert_eq!(string.get_last_n_chars(14), string);

        let str: &str = "♥foo よção♥ bar";
        assert_eq!(str.get_last_n_chars(9), "よção♥ bar");
        assert_eq!(str.get_last_n_chars(14), str);
    ```
    */
    fn get_last_n_chars(&self, num: usize) -> &str;

    /**
    Convert a string of digits to an vector of digits.
    ```
        use claudiofsr_lib::StrExtension;
        let text1: &str = "12345";
        let text2: &str = "ab1c";
        let text3: &str = "";

        let result1: Vec<u32> = text1.to_digits();
        let result2: Vec<u32> = text2.to_digits();
        let result3: Vec<u32> = text3.to_digits();

        assert_eq!(result1, vec![1, 2, 3, 4, 5]);
        assert_eq!(result2, [1]);
        assert_eq!(result3, []);
    ```
    <https://stackoverflow.com/questions/43516351/how-to-convert-a-string-of-digits-into-a-vector-of-digits>
    */
    fn to_digits(&self) -> Vec<u32>;

    /**
    Format CNPJ (ASCII alphanumeric with 14 characters)
    ```
        use claudiofsr_lib::StrExtension;
        let cnpj: &str = "12ABC678901234";
        assert_eq!(
            cnpj.format_cnpj(),
            "12.ABC.678/9012-34"
        );
    ```
    */
    fn format_cnpj(&self) -> String;

    /**
    Format CPF (ASCII alphanumeric with 11 characters)
    ```
        use claudiofsr_lib::StrExtension;
        let cpf: &str = "123ABC78901";
        assert_eq!(
            cpf.format_cpf(),
            "123.ABC.789-01"
        );
    ```
    */
    fn format_cpf(&self) -> String;

    /**
    Format NCM (ASCII alphanumeric with 8 characters)
    ```
        use claudiofsr_lib::StrExtension;
        let ncm: &str = "2309AB90";
        assert_eq!(
            ncm.format_ncm(),
            "2309.AB.90"
        );
    ```
    */
    fn format_ncm(&self) -> String;
}

impl<T> StrExtension for T
where
    T: Deref<Target = str>,
{
    // Output: usize

    fn chars_count(&self) -> usize {
        self.chars().count()
    }

    fn count_char(&self, ch: char) -> usize {
        self.chars()
            .filter(|current_char| *current_char == ch)
            .count()
    }

    // Output: bool

    fn contains_only_digits(&self) -> bool {
        !self.is_empty() && self.bytes().all(|x| x.is_ascii_digit())
    }

    fn contains_some_digits(&self) -> bool {
        self.bytes().any(|x| x.is_ascii_digit())
    }

    fn contains_num_digits(&self, num_digit: usize) -> bool {
        // self.chars().filter(|c| c.is_ascii_digit()).count() == num_digit
        self.chars_count() == num_digit && self.bytes().all(|x| x.is_ascii_digit())
    }

    fn contains_up_to_num_digits(&self, num_digit: usize) -> bool {
        self.chars_count() <= num_digit && self.bytes().all(|x| x.is_ascii_digit())
    }

    fn is_ascii_alphanumeric(&self) -> bool {
        self.chars().all(|c| c.is_ascii_alphanumeric())
    }

    fn is_alphanumeric(&self) -> bool {
        self.chars().all(char::is_alphanumeric)
    }

    // Output: String

    fn replace_multiple_whitespaces(&self) -> String {
        let s = self.deref();

        // TECHNIQUE 1: Fast-Path (SIMD-Accelerated Search)
        // The standard library's `contains` is highly optimized, often using SIMD
        // instructions to scan bytes. Since the vast majority of SPED fields are
        // already clean, this early return bypasses the manual loop overhead entirely
        // and avoids any temporary buffer logic.
        if !s.contains("  ") {
            return s.to_string();
        }

        // TECHNIQUE 2: Capacity Reservation
        // We pre-allocate a String with the original length. This ensures zero
        // reallocations (resizing) during the construction of the result,
        // even though the final string will likely be shorter.
        let mut new_string = String::with_capacity(s.len());
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut last_was_space = false;

        // TECHNIQUE 3: Single-Pass Byte Iteration
        // In UTF-8, the ASCII space (0x20) is unique and never appears as a tail byte
        // or component of a multi-byte character sequence. This allows us to safely
        // process raw bytes, avoiding the computational overhead of UTF-8
        // decoding (`chars()` iterator).
        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b' ' {
                if last_was_space {
                    // Redundant space detected.
                    // 1. "Flush" the accumulated valid text BEFORE this redundant space.
                    // Using `push_str` on a slice leverages `memcpy`, which is significantly
                    // faster than pushing individual characters or bytes.
                    if i > start {
                        new_string.push_str(&s[start..i]);
                    }
                    // 2. Advance the 'start' cursor to skip this specific redundant space byte.
                    start = i + 1;
                } else {
                    // This is the first space in a potential sequence.
                    // We mark the state but keep this space in the next valid slice flush.
                    last_was_space = true;
                }
            } else {
                // Non-space character encountered; reset the space tracking state.
                last_was_space = false;
            }
        }

        // TECHNIQUE 4: Tail Append
        // Append the remaining characters after the last processed multi-space sequence.
        if start < bytes.len() {
            new_string.push_str(&s[start..]);
        }

        new_string
    }

    fn remove_non_digits(&self) -> String {
        self.chars().filter(|c| c.is_ascii_digit()).collect()
    }

    fn remove_first_and_last_char(&self) -> String {
        let mut chars = self.chars();
        chars.next();
        chars.next_back();
        chars.collect()
    }

    fn select_first_digits(&self) -> String {
        self.chars()
            .map_while(|x| x.is_ascii_digit().then_some(x))
            .collect::<String>()
    }

    // Output: &str

    fn retain_first_digits(&self) -> &str {
        let mut index = 0;

        for (idx, c) in self.char_indices() {
            if !c.is_ascii_digit() {
                index = idx;
                break;
            }
        }

        &self[..index]
    }

    fn strip_prefix_and_sufix(&self, delimiter_byte: u8) -> &str {
        // ASCII is an 8-bit code. That is, it uses eight bits to represent
        // a letter or a punctuation mark. Eight bits are called a byte.
        let from = match self.bytes().position(|b| b == delimiter_byte) {
            Some(i) => i + 1,
            None => return self,
        };
        let to = self.bytes().rposition(|b| b == delimiter_byte).unwrap();
        //println!("self: {self} ; from: {from} ; to: {to}");
        &self[from..to]
    }

    fn get_first_n_chars(&self, num: usize) -> &str {
        //self.chars().take(num).collect()
        match self.char_indices().nth(num) {
            Some((split_pos, _character)) => &self[..split_pos],
            None => self,
        }
    }

    fn get_last_n_chars(&self, num: usize) -> &str {
        match self.char_indices().nth_back(num - 1) {
            Some((split_pos, _character)) => &self[split_pos..],
            None => self,
        }
    }

    // Output: Vec<u32>

    fn to_digits(&self) -> Vec<u32> {
        self.chars()
            //.map(|ch| ch.to_digit(10))
            //.collect::<Option<Vec<u32>>>()
            //.unwrap_or_default()
            .flat_map(|ch| ch.to_digit(10))
            .collect::<Vec<u32>>()
    }

    // Format ASCII alphanumeric

    fn format_cnpj(&self) -> String {
        if self.chars().count() == 14 && self.is_ascii_alphanumeric() {
            let formated: String = [
                &self[0..2],
                ".",
                &self[2..5],
                ".",
                &self[5..8],
                "/",
                &self[8..12],
                "-",
                &self[12..],
            ]
            .concat();
            formated
        } else {
            self.to_string()
        }
    }

    fn format_cpf(&self) -> String {
        if self.chars().count() == 11 && self.is_ascii_alphanumeric() {
            let formated: String = [
                &self[0..3],
                ".",
                &self[3..6],
                ".",
                &self[6..9],
                "-",
                &self[9..],
            ]
            .concat();
            formated
        } else {
            self.to_string()
        }
    }

    fn format_ncm(&self) -> String {
        if self.chars().count() == 8 && self.is_ascii_alphanumeric() {
            let formated: String = [&self[0..4], ".", &self[4..6], ".", &self[6..8]].concat();
            formated
        } else {
            self.to_string()
        }
    }
}

#[cfg(test)]
mod functions {
    use super::*;

    // cargo test -- --help
    // cargo test -- --nocapture
    // cargo test -- --show-output

    #[test]
    fn test_replace_multiple_whitespaces() {
        // cargo test -- --show-output test_replace_multiple_whitespaces
        let strings: Vec<&str> = vec![
            "🦀",
            " teste",
            "teste ",
            " teste ",
            "  teste",
            "teste  ",
            "  teste  ",
            "tes te",
            "tes  te",
            "tes   te",
            " tes te",
            "tes  te ",
            " tes  te ",
            "  tes te",
            "tes  te  ",
            "  tes  te  ",
            " ",
            "  ",
            "   ",
            "    ",
        ];
        for string in strings {
            let s = ["'", string, "'"].concat();
            println!("{:13} --> '{}'", s, string.replace_multiple_whitespaces());
        }
        let s1 = "tes  te".replace_multiple_whitespaces();
        let s2 = " tes  te".replace_multiple_whitespaces();
        let s3 = "tes  te ".replace_multiple_whitespaces();
        let s4 = " tes  te ".replace_multiple_whitespaces();
        let s5 = "  tes  te".replace_multiple_whitespaces();
        let s6 = "tes  te  ".replace_multiple_whitespaces();
        let s7 = "  tes  te  ".replace_multiple_whitespaces();
        let s8 = "         ".replace_multiple_whitespaces();

        assert_eq!(s1, "tes te");
        assert_eq!(s2, " tes te");
        assert_eq!(s3, "tes te ");
        assert_eq!(s4, " tes te ");
        assert_eq!(s5, " tes te");
        assert_eq!(s6, "tes te ");
        assert_eq!(s7, " tes te ");
        assert_eq!(s8, " ");
    }

    #[test]
    fn test_replace_multiple_whitespaces_with_multibyte_integrity() {
        // 1. The "Ã" Test (2-byte character)
        // Byte representation: [65, 195, 131, 79]
        let input = "AÇÃO     2024";
        let result = input.replace_multiple_whitespaces();
        assert_eq!(result, "AÇÃO 2024");
        assert_eq!(
            result.chars().count(),
            9,
            "Should have exactly 9 characters"
        );

        // 2. The Emoji Test (4-byte character) - Most likely to break byte-parsers
        // 🚀 is [240, 159, 154, 128]
        let input = "System   🚀   Online";
        let result = input.replace_multiple_whitespaces();
        assert_eq!(result, "System 🚀 Online");

        // If corrupted by 'b as char', the length would be higher than expected
        // because 1 emoji (4 bytes) would become 4 invalid characters.
        assert_eq!(
            result.len(),
            18,
            "Byte length should be 18 (12 ASCII + 4 Emoji + 2 Spaces)"
        );

        // 3. The Euro Test (3-byte character)
        // € is [226, 130, 172]
        let input = "Price:   100€   only!";
        let result = input.replace_multiple_whitespaces();
        assert_eq!(result, "Price: 100€ only!");
        assert_eq!(result.len(), 19);

        // 4. Combined Stress Test
        let input = "  SÃO   PAULO  🚀  €  ";
        let result = input.replace_multiple_whitespaces();
        // Note: leading/trailing spaces are preserved if they are single,
        // collapsed if multiple.
        assert_eq!(result, " SÃO PAULO 🚀 € ");
    }

    #[test]
    fn test_replace_multiple_whitespaces_with_specific_byte_boundaries() {
        // This test ensures we don't slice in the middle of a multi-byte char
        // by placing multiple spaces immediately after a 2-byte char.
        let input = "ABCÁ     DEF";
        // Á is [195, 129]. If the pointer logic is wrong, it might try to
        // read a space at the wrong offset.
        let result = input.replace_multiple_whitespaces();
        assert_eq!(result, "ABCÁ DEF");
    }

    #[test]
    fn test_select_first_digits() {
        // cargo test -- --show-output test_select_first_digits
        let strings: Vec<&str> = vec![
            "1234🦀",
            "1191-1",
            "10845/a",
            "987654Cláudio",
            "1",
            "a",
            "12345",
            "12345___abc",
        ];
        let digits: Vec<&str> = vec!["1234", "1191", "10845", "987654", "1", "", "12345", "12345"];

        //How to iterate through two arrays at once?
        for (&string, &digit) in strings.iter().zip(digits.iter()) {
            let s = ["'", string, "'"].concat();
            println!("{:15} --> '{}'", s, string.select_first_digits());
            assert_eq!(string.select_first_digits(), digit);
        }
    }

    #[test]
    fn test_contains_only_digits() {
        // cargo test -- --show-output test_contains_only_digits
        let strings: Vec<&str> = vec![
            "🦀", "12345", "12345x", " 12345", " 12345 ", "", " ", "0", "7", "10",
        ];
        for string in strings {
            let s = ["'", string, "'"].concat();
            println!("{:13} --> {}", s, string.contains_only_digits());
        }
        let s1 = "🦀".contains_only_digits();
        let s2 = "12345".contains_only_digits();
        let s3 = "12345x".contains_only_digits();
        let s4 = " 12345".contains_only_digits();
        let s5 = " 12345 ".contains_only_digits();
        let s6 = "".contains_only_digits();
        let s7 = " ".contains_only_digits();
        let s8 = "0".contains_only_digits();
        let s9 = "10".contains_only_digits();

        assert!(!s1);
        assert!(s2);
        assert!(!s3);
        assert!(!s4);
        assert!(!s5);
        assert!(!s6);
        assert!(!s7);
        assert!(s8);
        assert!(s9);
    }

    #[test]
    fn test_chars_count() {
        // cargo test -- --show-output test_chars_count
        let strings: Vec<&str> = vec![
            "🦀",
            "12345",
            "Cláudio",
            " Cláudio 🦀 çṕ@",
            "Bom dia おはよう!",
        ];
        for string in strings {
            let s = ["'", string, "'"].concat();
            println!("{} --> {}", s, string.chars_count());
        }
        let s1 = "🦀".chars_count();
        let s2 = "12345".chars_count();
        let s3 = "Cláudio".chars_count();
        let s4 = " Cláudio 🦀 çṕ@".chars_count();
        let s5 = "Bom dia おはよう!".chars_count();

        assert_eq!(s1, 1);
        assert_eq!(s2, 5);
        assert_eq!(s3, 7);
        assert_eq!(s4, 14);
        assert_eq!(s5, 13);
    }
}
