/// Trait extension to String
pub trait StringExtension {
    fn remove_all_whitespace(&mut self);
    fn remove_all_char(&mut self, c: char);
    fn get_first_n_chars(self, num: usize) -> String;
    fn get_last_n_chars(self, num: usize) -> String;
}

impl StringExtension for String {
    /**
    Remove all whitespace from a string.
    ```
        use claudiofsr_lib::StringExtension;

        let mut string = String::from(" for  bar \n");
        string.remove_all_whitespace();

        assert_eq!(string, "forbar");
    ```
    */
    fn remove_all_whitespace(&mut self) {
        self.retain(|c| !c.is_whitespace());
    }

    /**
    Remove all char from a string.
    ```
        use claudiofsr_lib::StringExtension;

        let mut string = String::from("for bar bbar");
        string.remove_all_char('b');

        assert_eq!(string, "for ar ar");
    ```
    */
    fn remove_all_char(&mut self, ch: char) {
        self.retain(|c| c != ch);
    }

    /**
    Get the first n character of a String.
    ```
        use claudiofsr_lib::StringExtension;

        let text = String::from("♥foo よção♥ bar").get_first_n_chars(10);
        assert_eq!(text, "♥foo よção♥");
    ```
    */
    fn get_first_n_chars(self, num: usize) -> String {
        self.chars().take(num).collect()
    }

    /**
    Get the last n character of a String.
    ```
        use claudiofsr_lib::StringExtension;

        let text = String::from("♥foo よção♥ bar").get_last_n_chars(9);
        assert_eq!(text, "よção♥ bar");
    ```
    */
    fn get_last_n_chars(self, num: usize) -> String {
        let length = self.chars().count();
        let minimum = length.min(num);
        //println!("length: {length}; minimum: {minimum}");
        //println!("chars: {:?}", self.chars());
        
        // attempt to subtract without overflow
        self.chars().skip(length - minimum).collect()
    }
}

/// Trait extension to &str
pub trait StrExtension {
    fn chars_count(self) -> usize;

    // Output: bool
    fn contains_only_digits(self) -> bool;
    fn contains_some_digits(self) -> bool;
    fn contains_num_digits(self, num_digit: usize) -> bool;
    fn contains_up_to_num_digits(self, num_digit: usize) -> bool;
    
    // Output: String
    fn replace_multiple_whitespaces(self) -> String;
    fn remove_non_digits(self) -> String;
    fn remove_first_and_last_char(self) -> String;
    fn select_first_digits(self) -> String;

    // Output: &str
    fn get_last_n_chars(&self, num: usize) -> &str;
    fn retain_first_digits(&self) -> &str;
    fn strip_prefix_and_sufix(&self, delimiter_byte: u8) -> &str;

    fn count_char(self, ch: char) -> usize;
    fn to_digits(self) -> Vec<u32>;

    // format
    fn format_cnpj(self) -> String;
    fn format_cpf(self) -> String;
    fn format_ncm(self) -> String;
}

impl StrExtension for &str {
    /**
    Returns the characters count.

    Not use len()
    ```
        use claudiofsr_lib::StrExtension;
        let text_a: &str = "12x45";
        let text_b: &str = "Bom dia おはよう!";
        let text_c: &str = " Cláudio 🦀 çṕ@";
        assert_eq!(text_a.chars_count(), 5);
        assert_eq!(text_b.chars_count(), 13);
        assert_eq!(text_c.chars_count(), 14);
    ```
    */
    fn chars_count(self) -> usize {
        self.chars().count()
    }

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
    fn contains_only_digits(self) -> bool {
        !self.is_empty() &&
        self.bytes().all(|x| x.is_ascii_digit())
    }

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
    fn contains_some_digits(self) -> bool {
        self.bytes().any(|x| x.is_ascii_digit())
    }

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
    fn contains_num_digits(self, num_digit: usize) -> bool {
        self.chars_count() == num_digit &&
        self.bytes().all(|x| x.is_ascii_digit())
    }

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
    fn contains_up_to_num_digits (self, num_digit: usize) -> bool {
        self.chars_count() <= num_digit &&
        self.bytes().all(|x| x.is_ascii_digit())
    }

    /**
    Replace multiple whitespace with a single one.
    ```
        use claudiofsr_lib::StrExtension;
        let text_a: &str = "a  bc d";
        let text_b: &str = "a   bc    d";
        let text_c: &str = "  a  bc d  ";
        let result1 = "a bc d";
        let result2 = " a bc d ";
        assert_eq!(result1, text_a.replace_multiple_whitespaces());
        assert_eq!(result1, text_b.replace_multiple_whitespaces());
        assert_eq!(result2, text_c.replace_multiple_whitespaces());
    ```
    */
    fn replace_multiple_whitespaces(self) -> String {
        let mut new_str: String = self.to_string();
        let mut previous_char: char = 'x'; // some non-whitespace character
        new_str.retain(|current_char| {
            //let keep: bool = !(previous_char == ' ' && current_char == ' ');
            let keep: bool = previous_char != ' ' || current_char != ' ';
            previous_char = current_char;
            keep
        });
        new_str
    }

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
    fn remove_non_digits(self) -> String {
        self
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect()
    }

    /// Remove the first and last character from a string
    ///
    /// <https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust>
    fn remove_first_and_last_char(self) -> String {
        let mut chars = self.chars();
        chars.next();
        chars.next_back();
        chars.collect()
    }

    /// Capture or Retain only the first group of digits:
    ///
    /// ```
    ///     use claudiofsr_lib::StrExtension;
    ///
    ///     let text01: &str = "1191-1";
    ///     let result: String = text01.select_first_digits();
    ///     assert_eq!(result, "1191");
    ///
    ///     let text02: &str = "10845/a";
    ///     let result: String = text02.select_first_digits();
    ///     assert_eq!(result, "10845");
    ///
    /// ```
    fn select_first_digits(self) -> String {
        self
            .chars()
            .map_while(|x| x.is_ascii_digit().then_some(x))
            .collect::<String>()
    }

    /**
    Get the last n character of a &str.
    ```
        use claudiofsr_lib::StrExtension;

        let text: &str = "for bar bbar ção".get_last_n_chars(7);
        assert_eq!(text, "bar ção");
    ```
    */
    fn get_last_n_chars(&self, num: usize) -> &str {
        let length = self.chars().count();
        &self[(length - num) ..]
    }

    /**
    Retain the first digits
    ```
        use claudiofsr_lib::StrExtension;
        let word = "12345abc678";
        let digits = word.retain_first_digits();

        assert_eq!(digits, "12345");
    ```
    */
	fn retain_first_digits(&self) -> &str {
		let mut index = 0;

		for (idx, c) in self.char_indices() {
			if !c.is_ascii_digit() {
				index = idx;
				break;
			}
		};

		&self[..index]
	}

    /// Returns a string with the prefix and suffix delimiter removed.
    ///
    /// <https://doc.rust-lang.org/src/core/str/mod.rs.html>
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

    /// Counts the number of occurrences of a given character in a String.
    ///
    /// ```
    ///     use claudiofsr_lib::StrExtension;
    ///
    ///     let line: &str = "|C170|foo|bar|zzz|";
    ///     let result: usize = line.count_char('|');
    ///     assert_eq!(result, 5);
    ///
    /// ```
    fn count_char(self, ch: char) -> usize {
        let mut new_str: String = self.to_owned();
        new_str.retain(|current_char| current_char == ch);
        //println!("pipes: '{new_str}' ; size: {}", new_str.len());
        new_str.len()
    }

    /// Convert a string of digits to an vector of digits
    ///
    /// <https://stackoverflow.com/questions/43516351/how-to-convert-a-string-of-digits-into-a-vector-of-digits>
    fn to_digits(self) -> Vec<u32> {
        let opt_vec: Option<Vec<u32>> = self
			.chars()
			.map(|ch| ch.to_digit(10))
			.collect();

        match opt_vec {
            Some(vec_of_digits) => vec_of_digits,
            None => vec![],
        }
    }
 
    /**
    Format CNPJ
    ```
        use claudiofsr_lib::StrExtension;
        let cnpj: &str = "12345678901234";
        assert_eq!(
            cnpj.format_cnpj(),
            "12.345.678/9012-34"
        );
    ```
    */
    fn format_cnpj(self) -> String
    {
        if self.contains_num_digits(14) {
            let formated: String = [
                &self[ 0 .. 2], ".",
                &self[ 2 .. 5], ".",
                &self[ 5 .. 8], "/",
                &self[ 8 .. 12], "-",
                &self[12 .. ]
            ].concat();
            formated
        } else {
            self.to_string()
        }
    }

    /**
    Format CPF
    ```
        use claudiofsr_lib::StrExtension;
        let cpf: &str = "12345678901";
        assert_eq!(
            cpf.format_cpf(),
            "123.456.789-01"
        );
    ```
    */
    fn format_cpf(self) -> String
    {
        if self.contains_num_digits(11) {
            let formated: String = [
                &self[0 .. 3], ".",
                &self[3 .. 6], ".",
                &self[6 .. 9], "-",
                &self[9 ..  ]
            ].concat();
            formated
        } else {
            self.to_string()
        }
    }

    /**
    Format NCM
    ```
        use claudiofsr_lib::StrExtension;
        let ncm: &str = "23099090";
        assert_eq!(
            ncm.format_ncm(),
            "2309.90.90"
        );
    ```
    */
    fn format_ncm(self) -> String
    {
        if self.contains_num_digits(8) {
            let formated: String = [
                &self[0 .. 4], ".",
                &self[4 .. 6], ".",
                &self[6 .. 8]
            ].concat();
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
            " teste", "teste ", " teste ",
            "  teste", "teste  ", "  teste  ",
            "tes te", "tes  te", "tes   te",
            " tes te", "tes  te ", " tes  te ",
            "  tes te", "tes  te  ", "  tes  te  ",
            " ", "  ", "   ", "    ",
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
    fn test_select_first_digits() {
        // cargo test -- --show-output test_select_first_digits
        let strings: Vec<&str> = vec![
            "1234🦀", "1191-1",
            "10845/a", "987654Cláudio",
            "1", "a", "12345",
            "12345___abc",
        ];
        let digits: Vec<&str> = vec![
            "1234", "1191",
            "10845", "987654",
            "1", "", "12345",
            "12345",
        ];

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
            "🦀", "12345",
            "12345x", " 12345",
            " 12345 ", "", " ",
            "0", "7", "10",
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
            "🦀", "12345",
            "Cláudio", " Cláudio 🦀 çṕ@",
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
