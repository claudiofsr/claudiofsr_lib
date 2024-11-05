use std::{
    error::Error,
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
        use claudiofsr_lib::{FileExtension, open_file};
        use std::{fs::File, io::Write, path::Path, error::Error};

        fn main() -> Result<(), Box<dyn Error>> {

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
    fn count_lines(&mut self) -> Result<u64, Box<dyn Error>>;
}

impl FileExtension for File {
    fn count_lines(&mut self) -> Result<u64, Box<dyn Error>> {
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
The `SkipBack` trait is a generic interface that allows you to modify the behavior
of an iterator by skipping a specified number of elements from its end.

It provides two methods:

- `skip_last()`, which removes the last element of the iterator,

- `skip_back(n)`, which removes the last `n` elements of the iterator.
*/
pub trait SkipBack {
    /**
    Returns an iterator that skips the last element of the original iterator.

    ### Examples

    ```
    use claudiofsr_lib::SkipBack;

        let iter = 1..=5;
        let data1: Vec<_> = iter.skip_last().collect();
        assert_eq!(data1, [1, 2, 3, 4] );

        let data2: Vec<_> = [1, 2, 3, 4, 5]
            .into_iter()
            .skip(1)
            .skip_last()
            .skip(1)
            .collect();
        assert_eq!(data2, [3, 4] );

        let data3: Vec<_> = [1, 2, 3]
            .into_iter()
            .skip_last()
            .skip_last()
            .skip_last()
            .collect();
        assert!(data3.is_empty());
    ```
    */
    fn skip_last(self) -> Self;

    /**
    Returns an iterator that skips the last 'n' elements of the original iterator.

    - n = 0: skip_back(0) returns the original iterator,

    - n = 1: skip_back(1) skips the last element,

    - n = 2: skip_back(2) skips the last 2 elements,

    - n = 3 and beyond: it continues to skip elements.

    ### Examples

    ```
    use claudiofsr_lib::SkipBack;

    let data1: Vec<_> = [1, 2, 3, 4, 5]
        .into_iter()
        .skip_back(2)
        .collect();
    assert_eq!(data1, [1, 2, 3]);

    let data2: Vec<_> = [1, 2, 3, 4, 5]
        .into_iter()
        .skip_back(2)
        .skip(1)
        .collect();
    assert_eq!(data2, [2, 3]);

    let data3: Vec<_> = [1, 2, 3, 4, 5]
        .into_iter()
        .skip(1)
        .skip_back(1)
        .skip_back(2)
        .skip(1)
        .collect();
    assert_eq!(data3, []);

    let iter4 = 1..=10;
    let data4: Vec<_> = iter4.skip_back(6).collect();
    assert_eq!(data4, [1, 2, 3, 4]);

    let iter5 = 1..=10;
    let data5: Vec<_> = iter5.skip_back(10).collect();
    assert_eq!(data5, []);
    ```
    */
    fn skip_back(self, n: usize) -> Self;
}

impl<I> SkipBack for I
where
    I: DoubleEndedIterator,
{
    fn skip_last(mut self) -> Self {
        self.nth_back(0);
        self
    }

    fn skip_back(mut self, n: usize) -> Self {
        if n > 0 {
            self.nth_back(n - 1);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn test_skip_last() {
        let iter = 1..=5;
        let data: Vec<_> = iter.skip_last().collect();
        assert_eq!(data, [1, 2, 3, 4]);
    }

    #[test]
    fn test_skip_last_and_skip() {
        let iter = 1..=5u64;
        let data: Vec<_> = iter.skip_last().skip(1).collect();
        assert_eq!(data, [2, 3, 4]);
    }

    #[test]
    fn test_skip_and_skip_last() {
        let iter = 1..=5u16;
        let data: Vec<_> = iter.skip(1).skip_last().collect();
        assert_eq!(data, [2, 3, 4]);
    }

    #[test]
    fn test_skip_back() {
        let data1: Vec<_> = [1, 2, 3, 4, 5].into_iter().skip_back(0).collect();
        assert_eq!(data1, [1, 2, 3, 4, 5]);

        let data2: Vec<_> = [1, 2, 3, 4, 5].into_iter().skip_back(1).collect();
        assert_eq!(data2, [1, 2, 3, 4]);

        let data3: Vec<_> = [1, 2, 3, 4, 5].into_iter().skip_back(2).collect();
        assert_eq!(data3, [1, 2, 3]);

        let iter4 = 1..=5;
        let data4: Vec<_> = iter4.skip_back(2).collect();
        assert_eq!(data4, [1, 2, 3]);

        let iter5 = 1..=10;
        let data5: Vec<_> = iter5.skip_back(6).collect();
        assert_eq!(data5, [1, 2, 3, 4]);

        let iter6 = 1..=10;
        let data6: Vec<_> = iter6.skip_back(11).collect();
        assert!(data6.is_empty());

        let data7: Vec<_> = [1, 2, 3, 4, 5, 6, 7, 8]
            .into_iter()
            .skip(1)
            .skip_back(3)
            .skip_back(1)
            .skip(1)
            .collect();
        assert_eq!(data7, [3, 4]);
    }

    #[test]
    fn test_skip_last_empty() {
        // Creates an iterator that yields nothing.
        let nope = iter::empty::<i32>();

        let data: Vec<_> = nope.skip_last().collect();
        assert_eq!(data, []);
    }

    #[test]
    fn test_skip_back_empty() {
        // Creates an iterator that yields nothing.
        let nope = iter::empty::<i32>();

        let data: Vec<_> = nope.skip_back(0).collect();
        assert_eq!(data, []);
    }
}

/*
pub trait IteratorBack: DoubleEndedIterator + Sized {
    fn skip_last(self) -> SkipBack<Self> {
        SkipBack::new(self, 1)
    }

    fn skip_back(self, n: usize) -> SkipBack<Self> {
        SkipBack::new(self, n)
    }
}

/// A custom iterator that skips elements from the end of the original iterator.
pub struct SkipBack<I> {
    /// The underlying iterator.
    iter: I,
    /// The number of elements to skip from the end.
    n: usize,
}

impl<I> SkipBack<I> {
    /// Creates a new `SkipBack` iterator with the specified number of elements to skip from the end.
    fn new(iter: I, n: usize) -> SkipBack<I> {
        SkipBack { iter, n }
    }
}

impl<I> Iterator for SkipBack<I>
where
    I: DoubleEndedIterator,
{
    type Item = I::Item;

    /// Advances the iterator by 1 element and decrements `n`.
    fn next(&mut self) -> Option<I::Item> {
        while self.n > 0 {
            self.iter.next_back();
            self.n -= 1;
        }
        self.iter.next()
    }
}

impl<I: DoubleEndedIterator> IteratorBack for I {}
*/
