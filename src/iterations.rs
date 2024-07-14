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
