use crate::MyResult;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

    assert!(matches!(counter, Ok(4)));
```

<https://www.reddit.com/r/rust/comments/wyk1l0/can_you_compose_stdiolines_with_stditercount/>
*/
pub trait IteratorExtension<E> {
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
    fn try_count(&mut self) -> Result<u64, E>;
}

impl<T, U, E> IteratorExtension<E> for T
where
    T: Iterator<Item = Result<U, E>>,
{
    fn try_count(&mut self) -> Result<u64, E> {
        self.try_fold(0, |accumulator: u64, element: Result<U, E>| {
            element.map(|_| accumulator + 1)
        })
    }
}

/// Adds a counter for the number of lines in a file.
pub trait FileExtension {
    /**
    Count the number of lines in the file.

    Example:
    ```
        use claudiofsr_lib::{FileExtension, open_file, MyResult};
        use std::{fs::File, io::Write, path::Path, error::Error};

        fn main() -> MyResult<()> {

            let lines = r"A test
            Actual content
            More content
            Another test";

            let filename = "/tmp/sample.txt";
            let mut file = File::create(filename)?;
            file.write_all(lines.as_bytes())?;

            let path = Path::new(filename);
            let mut file: File = open_file(path)?;
            let number_of_lines: u64 = file.count_lines()?;

            assert_eq!(number_of_lines, 4);
            Ok(())
        }
    ````
    */
    fn count_lines(&mut self) -> MyResult<u64>;
}

impl FileExtension for File {
    fn count_lines(&mut self) -> MyResult<u64> {
        let count: u64 = BufReader::new(self)
            //.lines()     // Return an error if the read bytes are not valid UTF-8
            .split(b'\n') // Ignores invalid UTF-8 but
            .try_count()?; // Catches other errors

        Ok(count)
    }

    /*
    /// Count the number of lines in the file
    ///
    /// use memmap2::Mmap;
    fn count_lines(&mut self) -> MyResult<u64> {

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
        .skip(1)
        .skip_back(2)
        .skip(2)
        .skip_back(1)
        .skip(1)
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

/// Tests for the `skip_last` method.
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
