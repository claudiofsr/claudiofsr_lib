use std::{fs::File, io};

#[cfg(not(feature = "fast-lines"))]
use std::io::{BufReader, Read};

// Importações condicionais apenas se a feature estiver ativa
#[cfg(feature = "fast-lines")]
use {memmap2::Mmap, rayon::prelude::*};

/// Extension trait adding utility methods for file manipulation.
pub trait FileExtension {
    /**
    Counts the number of lines in the file using the most efficient method available.

    ### Behavior
    It strictly counts the newline byte `0x0A` (`\n`), similar to the unix `wc -l` command.

    ### Characteristics:
    1. **Encoding-agnostic**: Works with UTF-8, ASCII, and legacy encodings (binary safe).
    2. **Memory Efficient**: Uses `BufReader` for buffered reads and iterates continuously
       without allocating memory for line strings (Zero-Allocation logic).
    3. **Side Effects**: The file cursor **will remain at the end of the file**.
       If you need to read the file again, you must manually `rewind` it.

    ### Implementations
    1. **Standard (Default)**: Uses `BufReader` and `try_fold`. Zero-allocation and safe.
    2. **Fast (Feature `fast-lines`)**: Uses `memmap2` and `rayon` for parallel processing.
       This creates a memory map of the file and counts newlines using multiple CPU threads.
       *Warning*: This involves `unsafe` code internally.

    ### Example:
    ```
    use claudiofsr_lib::FileExtension;
    use std::{fs::File, io::{Seek, Write}};

    fn main() -> std::io::Result<()> {
        let path = "/tmp/sample.txt";
        let mut file = File::create(path)?;

        // Note: The string ends with \n, so we expect 4 lines.
        let lines = "Line 1\nLine 2\nLine 3\nLine 4\n";

        file.write_all(lines.as_bytes())?;

        // Re-open the file for reading
        let mut file = File::open(path)?;

        // Count lines
        let number_of_lines: u64 = file.count_lines()?;

        // Optionally rewind the file cursor if you need to read it again
        file.rewind()?;

        // Cleanup
        std::fs::remove_file(path)?;

        assert_eq!(number_of_lines, 4);
        Ok(())
    }
    ```
    */
    fn count_lines(&self) -> io::Result<u64>;
}

// cargo test -- --show-output count_lines
// cargo test --features fast-lines -- --show-output count_lines

impl FileExtension for File {
    // -------------------------------------------------------------------------
    // Implementation 1: HIGH PERFORMANCE (Requires "fast-lines" feature)
    // -------------------------------------------------------------------------
    #[cfg(feature = "fast-lines")]
    fn count_lines(&self) -> io::Result<u64> {
        // Safety: Mmap is inherently unsafe because undefined behavior can occur
        // if the file is truncated or modified by another process while mapped.
        // However, for a cli utility reading a file, this is usually an acceptable risk.
        let mmap = unsafe { Mmap::map(self)? };

        // Rayon parallel iterator: splits the byte slice across threads
        // and counts occurrences of '\n' extremely fast.
        // We use `filter` instead of `split` to avoid semantic ambiguity and allocations.
        let count = mmap.par_iter().filter(|&&byte| byte == b'\n').count();

        Ok(count as u64)
    }

    // -------------------------------------------------------------------------
    // Implementation 2: STANDARD (Default - No extra dependencies)
    // -------------------------------------------------------------------------
    #[cfg(not(feature = "fast-lines"))]
    fn count_lines(&self) -> io::Result<u64> {
        // Wrap the file in a BufReader for efficient I/O (reduces syscalls).
        // Using a reference to self ensures we don't consume the File ownership.
        BufReader::new(self)
            .bytes() // Returns an iterator of Result<u8, io::Error>
            // try_fold is the functional equivalent of a reduce loop with error propagation.
            .try_fold(0u64, |acc, byte_result| {
                // Map the result: if Ok(byte), check for newline.
                // If Err, it propagates immediately.
                byte_result.map(|b| if b == b'\n' { acc + 1 } else { acc })
            })
    }
}

/**
The `IteratorBack` trait provides extension methods for iterators
to skip a specified number of elements from their end.

It provides two methods:

- `skip_last()`, which removes the last element of the iterator,

- `skip_back(n)`, which removes the last `n` elements of the iterator.
*/
pub trait IteratorBack {
    /**
    Returns an iterator that skips the last element of the original iterator.

    ### Examples

    ```
    use claudiofsr_lib::IteratorBack;

        let iter = 1..=5;
        let data1: Vec<_> = iter.skip_last().collect();
        assert_eq!(data1, [1, 2, 3, 4] );

        let data2: Vec<_> = [1, 2, 3, 4, 5, 6, 7]
            .into_iter()
            .skip(1)     // Skips from front
            .skip_last() // Skips from back
            .skip(1)
            .skip_last()
            .skip(1)
            .collect();
        assert_eq!(data2, [4, 5] );

        let data3: Vec<_> = "a|b|c|d|e"
            .split('|')
            .skip_last() // skip "e"
            .skip(1)     // skip "a"
            .collect();
        assert_eq!(data3, ["b", "c", "d"] );

        let data4: Vec<u64> = [1, 2]
            .into_iter()
            .skip_last()
            .skip_last()
            .collect();
        assert!(data4.is_empty());
    ```
    */
    fn skip_last(self) -> Self;

    /**
    Skip a specified number of elements from the end of the iterator.

    Returns a new iterator with the last `n` elements skipped.

    - `n = 0`: returns the original iterator (no elements skipped),

    - `n > 0`: skips `n` elements from the end.

    ### Examples

    ```
    use claudiofsr_lib::IteratorBack;

    let iter = 1..=5;
    let dados1: Vec<u64> = iter.skip_back(2).collect();
    assert_eq!(dados1, [1, 2, 3]);

    let dados2: Vec<_> = [1, 2, 3, 4, 5, 6, 7, 8, 9]
        .into_iter()
        .skip(1)        // [2, 3, 4, 5, 6, 7, 8, 9]
        .skip_back(2)   // [2, 3, 4, 5, 6, 7]
        .skip(2)        // [4, 5, 6, 7]
        .skip_back(1)   // [4, 5, 6]
        .skip(1)        // [5, 6]
        .collect();
    assert_eq!(dados2, [5, 6]);

    let dados3: Vec<_> = [1, 2, 3, 4, 5, 6, 7, 8, 9]
        .into_iter()
        .skip(3)
        .skip_back(4)
        .skip(3)
        .collect();
    assert_eq!(dados3, []);

    let line = "field_1 | field_2| field_3 |field_4 | field_5";
    let dados4: Vec<String> = line
        .split('|')
        .skip_back(2) // Skip the last 2 elements
        .skip(1)      // Skip the first element (field_1)
        .map(|field| field.trim().to_string())
        .collect();
    assert_eq!(dados4, ["field_2", "field_3"]);
    ```
    */
    fn skip_back(self, n: usize) -> Self;
}

// Implement the trait IteratorBack for all iterators that are also DoubleEndedIterator.
impl<I> IteratorBack for I
where
    I: DoubleEndedIterator,
{
    fn skip_last(mut self) -> Self {
        let _last = self.next_back();
        self
    }

    fn skip_back(mut self, n: usize) -> Self {
        if n > 0 {
            let _last_n = self.nth_back(n - 1);
        }
        self
    }
}

//----------------------------------------------------------------------------//
//                                   Tests                                    //
//----------------------------------------------------------------------------//
//
// cargo test -- --help
// cargo test -- --nocapture
// cargo test -- --show-output

/// Run tests with:
/// cargo test -- --show-output skip_last
#[cfg(test)]
mod test_skip_last {
    use super::*;
    use std::iter;

    #[test]
    fn test_empty() {
        let iter = iter::empty::<i32>();
        let result: Vec<_> = iter.skip_last().collect();
        assert_eq!(result, []);
    }

    #[test]
    fn test_single_element() {
        let data = [1];
        let result: Vec<_> = data.into_iter().skip_last().collect();
        assert_eq!(result, []);
    }

    #[test]
    fn test_multiple_elements() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result: Vec<_> = data
            .into_iter()
            .skip(1)
            .skip_last()
            .skip(1)
            .skip_last()
            .skip(1)
            .collect();
        assert_eq!(result, [4, 5, 6, 7]);
    }

    #[test]
    fn test_split_and_skip_last_and_skip() {
        let line = " | field_1| field_2 |field_3 | ";

        let data: Vec<_> = line
            .split('|')
            //.skip(1) // Skip the first element (empty string)
            .skip_last() // Skip the last element (empty string)
            .skip(1)
            .map(|field| field.trim().to_string())
            .collect();

        assert_eq!(data, ["field_1", "field_2", "field_3"]);
    }
}

/// Tests for the `skip_back` method.
#[cfg(test)]
mod test_skip_back {
    use super::*;
    use std::iter;

    #[test]
    fn test_empty() {
        let iter = iter::empty::<i32>();
        let result: Vec<_> = iter.skip_back(1).collect();
        assert_eq!(result, []);
    }

    #[test]
    fn test_single_element() {
        let data = [1];
        let result: Vec<_> = data.into_iter().skip_back(0).collect();
        assert_eq!(result, [1]);

        let data = [1];
        let result: Vec<_> = data.into_iter().skip_back(1).collect();
        assert_eq!(result, []);
    }

    #[test]
    fn test_multiple_elements() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result: Vec<_> = data
            .into_iter()
            .skip(1)
            .skip_back(1)
            .skip(1)
            .skip_back(2)
            .skip(1)
            .collect();
        assert_eq!(result, [4, 5, 6]);
    }

    #[test]
    fn test_split_and_skip_last_and_skip() {
        let line = " | field_1| field_2 |field_3 | ";

        let data: Vec<_> = line
            .split('|')
            //.skip(1) // Skip the first element (empty string)
            .skip_back(1) // Skip the last element (empty string)
            .skip(1)
            .map(|field| field.trim().to_string())
            .collect();

        assert_eq!(data, ["field_1", "field_2", "field_3"]);
    }
}

/// Run tests with:
/// cargo test -- --show-output count_lines_tests
/// cargo test --features fast-lines -- --show-output count_lines_tests
#[cfg(test)]
mod count_lines_tests {
    use super::*;
    use std::io::{Seek, Write};

    #[test]
    fn test_count_lines() -> io::Result<()> {
        // Setup: Create a temporary file path
        let path = "/tmp/sample_lines.txt";

        // Scope to ensure file is written and closed before reading
        {
            let mut file = File::create(path)?;
            // Note: The string ends with \n, so we expect 4 lines.
            let lines = "Line 1\nLine 2\nLine 3\nLine 4\n";
            println!("lines:\n{}", lines);
            file.write_all(lines.as_bytes())?;
        } // File is closed here

        // Act: Open for reading
        let mut file = File::open(path)?;

        // Count lines
        let number_of_lines: u64 = file.count_lines()?;

        // Optional: Rewind if we wanted to read again
        file.rewind()?;

        // Assert
        println!("Calculated lines: {}", number_of_lines);
        assert_eq!(number_of_lines, 4);

        // Cleanup
        std::fs::remove_file(path)?;
        Ok(())
    }
}
