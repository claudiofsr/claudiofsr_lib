use crate::MyResult;
use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashSet},
    fs::File,
    io::Write,
    path::Path,
};

/// Trait extension for HashSet
pub trait HashSetExtension<T> {
    /**
    Convert HashSet to a Vector with unique elements

    Example:
    ```
        use std::collections::HashSet;
        use claudiofsr_lib::HashSetExtension;

        let set1 = HashSet::from([2, 3, 1, 4, 5, 2]);
        let mut vec1 = set1.to_vec();
        println!("{:?}", vec1);

        let set2 = HashSet::from(["b", "a", "c", "a", "b"]);
        let mut vec2 = set2.to_vec();
        println!("{:?}", vec2);

        let mut vec3: Vec<u16> = [2, 3, 1, 4, 3, 3, 5, 2]
            .into_iter()
            .collect::<HashSet<_>>()
            .to_vec();

        vec1.sort();
        vec2.sort();
        vec3.sort();

        assert_eq!(vec1, [1, 2, 3, 4, 5]);
        assert_eq!(vec2, ["a", "b", "c"]);
        assert_eq!(vec3, [1, 2, 3, 4, 5]);
    ```
    */
    fn to_vec(&self) -> Vec<T>;

    /**
    Convert HashSet to a Vector with unique and ordered elements

    Example:
    ```
        use std::collections::HashSet;
        use claudiofsr_lib::HashSetExtension;

        let set1 = HashSet::from([2, 3, 1, 4, 5, 2]);
        let vec1 = set1.to_vec_sorted();
        println!("{:?}", vec1);

        let set2 = HashSet::from(["b", "a", "c", "a", "b"]);
        let vec2 = set2.to_vec_sorted();
        println!("{:?}", vec2);

        let vec3: Vec<u16> = [2, 3, 1, 4, 3, 3, 5, 2]
            .into_iter()
            .collect::<HashSet<_>>()
            .to_vec_sorted();

        assert_eq!(vec1, [1, 2, 3, 4, 5]);
        assert_eq!(vec2, ["a", "b", "c"]);
        assert_eq!(vec3, [1, 2, 3, 4, 5]);
    ```
    */
    fn to_vec_sorted(&self) -> Vec<T>
    where
        T: Ord;

    /// Write all HashSet elements to an output file.
    fn write_to_file<P>(&self, output_file: P) -> MyResult<()>
    where
        P: AsRef<Path>;
}

impl<T> HashSetExtension<T> for HashSet<T>
where
    T: Clone + std::fmt::Display,
{
    fn to_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }

    fn to_vec_sorted(&self) -> Vec<T>
    where
        T: Ord,
    {
        self.iter().sorted().cloned().collect()
    }

    fn write_to_file<P>(&self, output_file: P) -> MyResult<()>
    where
        P: AsRef<Path>,
    {
        // If the set is empty, return Ok() immediately
        if self.is_empty() {
            return Ok(());
        }

        // Create a new file with the given path
        let mut file = File::create(output_file)?;

        // Iterate over each element in the BTreeSet
        for elem in self {
            // Write each element to the file, followed by a newline character
            writeln!(file, "{elem}")?;
        }

        // Return Ok() to indicate that writing was successful
        Ok(())
    }
}

/// Trait extension for BTreeSet
pub trait BTreeSetExtension<T> {
    /**
    Convert BTreeSet to a Vector with unique and ordered elements

    Example:
    ```
        use std::collections::BTreeSet;
        use claudiofsr_lib::BTreeSetExtension;

        let set1 = BTreeSet::from([2, 3, 1, 4, 5, 2]);
        let vec1 = set1.to_vec();
        println!("{:?}", vec1);

        let set2 = BTreeSet::from(["b", "a", "c", "a", "b"]);
        let vec2 = set2.to_vec();
        println!("{:?}", vec2);

        let vec3: Vec<u16> = [2, 3, 1, 4, 3, 3, 5, 2]
            .into_iter()
            .collect::<BTreeSet<_>>()
            .to_vec();

        assert_eq!(vec1, [1, 2, 3, 4, 5]);
        assert_eq!(vec2, ["a", "b", "c"]);
        assert_eq!(vec3, [1, 2, 3, 4, 5]);
    ```
    */
    fn to_vec(&self) -> Vec<T>;

    /// Write all BTreeSet elements to an output file.
    fn write_to_file<P>(&self, output_file: P) -> MyResult<()>
    where
        P: AsRef<Path>;
}

impl<T> BTreeSetExtension<T> for BTreeSet<T>
where
    T: Clone + std::fmt::Display,
{
    fn to_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }

    fn write_to_file<P>(&self, output_file: P) -> MyResult<()>
    where
        P: AsRef<Path>,
    {
        // If the set is empty, return Ok() immediately
        if self.is_empty() {
            return Ok(());
        }

        // Create a new file with the given path
        let mut file = File::create(output_file)?;

        // Iterate over each element in the BTreeSet
        for elem in self {
            // Write each element to the file, followed by a newline character
            writeln!(file, "{elem}")?;
        }

        // Return Ok() to indicate that writing was successful
        Ok(())
    }
}
