use std::{cmp::Ord, collections::HashSet, hash::Hash, iter::Peekable};

/// Iterator Extension
pub trait IteratorExt: Iterator + Sized {
    /**
    Example:
    ```
        use claudiofsr_lib::IteratorExt;
        //use itertools::Itertools;

        let numbers = vec![1, 3, 2, 2, 5, 2, 3, 4];
        let unique_numbers: Vec<_> = numbers
            .into_iter()
            .get_unique()
            //.unique()
            .collect();

        assert_eq!(unique_numbers, &[1, 3, 2, 5, 4]);
    ```
    Font: My favorite Rust design pattern

    <https://www.youtube.com/watch?v=qrf52BVaZM8>

    <https://letsgetrusty.com/cheatsheet>
    */
    fn get_unique(self) -> UniqueIterator<Self> {
        UniqueIterator::new(self)
    }

    /**
    Returns an iterator that skips the last element of the original iterator.

    ### Examples

    ```
        use claudiofsr_lib::IteratorExt;

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
    <https://users.rust-lang.org/t/iterator-skip-last>
    */
    fn skip_last(self) -> SkipLastIterator<Self> {
        SkipLastIterator::new(self)
    }
}

pub struct UniqueIterator<I: Iterator> {
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I: Iterator> UniqueIterator<I> {
    fn new(iter: I) -> UniqueIterator<I> {
        UniqueIterator {
            iter,
            seen: HashSet::new(),
        }
    }
}

impl<I> Iterator for UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|item| self.seen.insert(item.clone()))
    }
}

// impl IteratorExt for std::vec::IntoIter<i32> {}
// impl IteratorExt for std::vec::IntoIter<i64> {}
// ...
impl<I: Iterator> IteratorExt for I {}

/// Get unique values from vector items.
pub trait UniqueElements {
    /**
    Get unique values from vector items.

    This method operates in place, visiting each element exactly once in the
    original order, and preserves the order of the retained elements.

    Example:
    ```
        use claudiofsr_lib::UniqueElements;

        let mut items1: Vec<u16> = Vec::new();
        let mut items2: Vec<u32> = vec![1, 3, 1, 2, 2, 4, 3];
        let mut items3: Vec<&str> = vec!["foo", "foo", "bar", "foo"];
        let mut items4: Vec<char> = vec!['f', 'o', 'o', ' ', 'b', 'a', 'r'];

        items1.unique();
        items2.unique();
        items3.unique();
        items4.unique();

        assert!(items1.is_empty());
        assert_eq!(items1, []);
        assert_eq!(items2, [1, 3, 2, 4]);
        assert_eq!(items3, ["foo", "bar"]);
        assert_eq!(items4, ['f', 'o', ' ', 'b', 'a', 'r']);
    ```
    `items.retain(|item| seen.insert(item.clone()))`

    This works because retain only keeps items for which the predicate returns true,
    and insert only returns true if the item was not previously present in the set.

    Since the vector is traversed in order, we end up keeping just the first occurrence of each item.

    <https://stackoverflow.com/questions/64819025/is-there-a-simple-way-remove-duplicate-elements-from-an-array>

    <https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain>
    */
    fn unique(&mut self);

    /**
    Get unique and ordered elements from `Vec<T>`.

    Example:
    ```
        use claudiofsr_lib::UniqueElements;

        let mut items1: Vec<u16> = Vec::new();
        let mut items2: Vec<u32> = vec![1, 3, 1, 2, 2, 4, 3];
        let mut items3: Vec<&str> = vec!["foo", "foo", "bar", "foo"];
        let mut items4: Vec<char> = vec!['f', 'o', 'o', ' ', 'b', 'a', 'r'];

        items1.unique_ordered();
        items2.unique_ordered();
        items3.unique_ordered();
        items4.unique_ordered();

        assert!(items1.is_empty());
        assert_eq!(items1, []);
        assert_eq!(items2, [1, 2, 3, 4]);
        assert_eq!(items3, ["bar", "foo"]);
        assert_eq!(items4, [' ', 'a', 'b', 'f', 'o', 'r']);
    ```
    */
    fn unique_ordered(&mut self);
}

impl<T> UniqueElements for Vec<T>
where
    T: Clone + Hash + Ord,
{
    fn unique(self: &mut Vec<T>) {
        let mut seen: HashSet<T> = HashSet::new();
        self.retain(|item| seen.insert(item.clone()));
    }

    fn unique_ordered(&mut self) {
        self.sort_unstable();
        self.dedup();
    }
}

/**
Define a SkipLastIterator struct to skip the last elements of another sequence.

<https://users.rust-lang.org/t/iterator-skip-last>
*/
pub struct SkipLastIterator<I: Iterator> {
    iter: Peekable<I>,
}

impl<I: Iterator> SkipLastIterator<I> {
    fn new(iter: I) -> SkipLastIterator<I> {
        SkipLastIterator {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator> Iterator for SkipLastIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        // Advances the iterator and returns the next value.
        let next_item = self.iter.next();

        // 'peek()' returns a reference to the next() value without advancing the iterator.
        // Get the next element without advancing the iterator (using peek())
        // to check if there are more elements).
        if self.iter.peek().is_some() {
            // If there's another item after next_item, return next_item.
            next_item
        } else {
            // Otherwise, return None to indicate that there are no more elements.
            None
        }
    }
}

#[cfg(test)]
mod filter_unique {
    use super::*;

    // cargo test -- --help
    // cargo test -- --nocapture
    // cargo test -- --show-output
    // cargo test -- --show-output filter_unique

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
    /// `cargo test -- --show-output remove_duplicates`
    ///
    /// rustfmt src/my_traits.rs
    fn remove_duplicates() {
        let mut elements = vec![1, 2, 4, 2, 5, 3, 2];
        println!("elements: {elements:?}");

        elements.unique_ordered();
        println!("elements.unique_ordered(): {elements:?}");

        assert_eq!(elements, vec![1, 2, 3, 4, 5])
    }
}
