use std::{cmp::Ord, collections::HashSet, hash::Hash, iter::Peekable};

/// A trait for extracting unique elements from a vector.
///
/// This trait adds methods to `Vec<T>` for removing duplicate elements,
/// either while preserving the original order (`unique`) or after sorting (`unique_ordered`).
///
/// ### Type Parameters
///
/// * `T`: The type of elements in the vector.
///   The required trait bounds depend on the method being used.
///     - `unique`: Requires `Eq`, `Hash`, and `Clone`.
///     - `unique_ordered`: Requires `Eq` and `Ord`.
///
/// ### Examples
///
/// To deduplicate while keeping the original order:
///
/// ```
/// use claudiofsr_lib::UniqueElements;
///
/// let mut vec = vec![3, 2, 2, 3, 5, 3, 4, 2, 1, 5];
/// vec.unique();
/// assert_eq!(vec, vec![3, 2, 5, 4, 1]);
/// ```
///
/// To deduplicate after sorting:
///
/// ```
/// use claudiofsr_lib::UniqueElements;
///
/// let mut vec = vec![3, 2, 2, 3, 5, 3, 4, 2, 1, 5];
/// vec.unique_ordered();
/// assert_eq!(vec, vec![1, 2, 3, 4, 5]);
/// ```
pub trait UniqueElements<T> {
    /// Deduplicates elements in the vector while preserving the original order.
    fn unique(&mut self)
    where
        T: Eq + Hash + Clone;

    /// Deduplicates elements in the vector after sorting them.
    fn unique_ordered(&mut self)
    where
        T: Eq + Ord;
}

impl<T> UniqueElements<T> for Vec<T> {
    /// Deduplicates elements in a vector while preserving the original order.
    ///
    /// This method iterates through the vector and keeps only the first occurrence
    /// of each element, effectively removing duplicates and maintaining the order in which
    /// elements first appear. It uses a `HashSet` to efficiently track seen elements.
    fn unique(&mut self)
    where
        T: Eq + Hash + Clone,
    {
        // `HashSet` to keep track of elements we've already encountered.
        let mut seen = HashSet::new();

        // `retain` iterates through the vector and keeps elements based on the closure's return value.
        self.retain(|x| {
            // `seen.insert(x.clone())` attempts to insert a clone of the current element `x` into the `HashSet`.
            // - If `x` is already in the `HashSet`, `insert` returns `false`.
            // - If `x` is NOT in the `HashSet`, `insert` inserts it and returns `true`.
            // We want to keep the element only if it's the first time we're seeing it (i.e., `insert` returns `true`).
            seen.insert(x.clone()) // Keep element if it's the first time we see it
        });
    }

    /// Sorts the elements and then removes the duplicated elements.
    /// The final vector will contain only unique elements in sorted order.
    fn unique_ordered(&mut self)
    where
        T: Eq + Ord,
    {
        self.sort_unstable();
        self.dedup();
    }
}

/// Extension trait for iterators, providing additional functionality.
pub trait IteratorExt: Iterator + Sized {
    /// Returns an iterator that yields only the unique elements from the original iterator,
    /// preserving the order in which they first appear.
    ///
    /// ### Examples
    ///
    /// ```
    /// use claudiofsr_lib::IteratorExt;
    ///
    /// let numbers = vec![1, 3, 2, 2, 5, 2, 3, 4];
    /// let unique_numbers: Vec<_> = numbers
    ///     .into_iter()
    ///     .get_unique()
    ///     //.unique()
    ///     .collect();
    ///
    /// assert_eq!(unique_numbers, &[1, 3, 2, 5, 4]);
    /// ```
    ///
    /// ### Source
    ///
    /// Inspired by: "My favorite Rust design pattern"
    /// - <https://www.youtube.com/watch?v=qrf52BVaZM8>
    ///
    /// - <https://letsgetrusty.com/cheatsheet>
    fn get_unique(self) -> UniqueIterator<Self> {
        UniqueIterator::new(self)
    }

    /// Returns an iterator that skips the last element of the original iterator.
    ///
    /// ### Examples
    ///
    /// ```
    /// use claudiofsr_lib::IteratorExt;
    ///
    /// let iter = 1..=5;
    /// let data1: Vec<_> = iter.skip_last().collect();
    /// assert_eq!(data1, [1, 2, 3, 4]);
    ///
    /// let data2: Vec<_> = [1, 2, 3, 4, 5]
    ///     .into_iter()
    ///     .skip(1)
    ///     .skip_last()
    ///     .skip(1)
    ///     .collect();
    /// assert_eq!(data2, [3, 4]);
    ///
    /// let data3: Vec<_> = [1, 2, 3]
    ///     .into_iter()
    ///     .skip_last()
    ///     .skip_last()
    ///     .skip_last()
    ///     .collect();
    /// assert!(data3.is_empty());
    /// ```
    ///
    /// ### Source
    ///
    /// Inspired by: <https://users.rust-lang.org/t/iterator-skip-last>
    fn skip_last(self) -> SkipLastIterator<Self> {
        SkipLastIterator::new(self)
    }
}

// Implement the IteratorExt trait for all types that implement the Iterator trait.
// impl IteratorExt for std::vec::IntoIter<i32> {}
// impl IteratorExt for std::vec::IntoIter<i64> {}
// ...
impl<I: Iterator> IteratorExt for I {}

/// An iterator that yields only the unique elements from an underlying iterator,
/// preserving the order in which they first appear.
pub struct UniqueIterator<I: Iterator> {
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I: Iterator> UniqueIterator<I> {
    /// Creates a new `UniqueIterator` from an existing iterator.
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

    /// Advances the iterator and returns the next value. Returns `None` when the end is reached.
    fn next(&mut self) -> Option<Self::Item> {
        // Find the next item in the iterator that hasn't been seen before.
        // If the iterator is exhausted, this will return `None`.
        self.iter.find(|item| self.seen.insert(item.clone()))
    }
}

/// An iterator that skips the last element of the underlying iterator.
pub struct SkipLastIterator<I: Iterator> {
    iter: Peekable<I>,
}

impl<I: Iterator> SkipLastIterator<I> {
    /// Creates a new `SkipLastIterator` from an existing iterator.
    fn new(iter: I) -> SkipLastIterator<I> {
        SkipLastIterator {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator> Iterator for SkipLastIterator<I> {
    type Item = I::Item;

    /// Advances the iterator and returns the next value, skipping the last element.
    /// Returns `None` when the end is reached or the last element is encountered.
    fn next(&mut self) -> Option<I::Item> {
        // Get the next item from the iterator.
        let next_item = self.iter.next();

        // Check if there are more elements after the current one using `peek()`.
        // 'peek()' returns a reference to the next() value without advancing the iterator.
        match self.iter.peek() {
            Some(_) => {
                // If there's another item after `next_item`, return `next_item`.
                next_item
            }
            None => {
                // If there are no more elements after `next_item`, it must be the last item, so return `None`.
                None
            }
        }
    }
}

#[cfg(test)]
mod tests_iterator_ext {
    use super::*;

    #[test]
    fn test_get_unique() {
        let numbers = vec![1, 3, 2, 2, 5, 2, 3, 4];
        let unique_numbers: Vec<_> = numbers.into_iter().get_unique().collect();
        assert_eq!(unique_numbers, &[1, 3, 2, 5, 4]);
    }

    #[test]
    fn test_get_unique_empty() {
        let numbers: Vec<i32> = vec![];
        let unique_numbers: Vec<_> = numbers.into_iter().get_unique().collect();
        assert_eq!(unique_numbers, &[]);
    }

    #[test]
    fn test_get_unique_all_same() {
        let numbers = vec![1, 1, 1, 1, 1];
        let unique_numbers: Vec<_> = numbers.into_iter().get_unique().collect();
        assert_eq!(unique_numbers, &[1]);
    }

    #[test]
    fn test_get_unique_strings() {
        let strings = vec!["a", "b", "b", "c", "a", "d", "c", "b", "e"];
        let unique_strings: Vec<_> = strings.into_iter().get_unique().collect();
        assert_eq!(unique_strings, &["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_skip_last() {
        let iter = 1..=5;
        let data1: Vec<_> = iter.skip_last().collect();
        assert_eq!(data1, [1, 2, 3, 4]);
    }

    #[test]
    fn test_skip_last_empty() {
        let iter: Vec<i32> = vec![];
        let data1: Vec<_> = iter.into_iter().skip_last().collect();
        assert_eq!(data1, []);
    }

    #[test]
    fn test_skip_last_one_element() {
        let iter = vec![1];
        let data1: Vec<_> = iter.into_iter().skip_last().collect();
        assert_eq!(data1, []);
    }

    #[test]
    fn test_skip_last_multiple_skips() {
        let data2: Vec<_> = [1, 2, 3, 4, 5]
            .into_iter()
            .skip(1)
            .skip_last()
            .skip(1)
            .collect();
        assert_eq!(data2, [3, 4]);
    }

    #[test]
    fn test_skip_last_chained_skips() {
        let data3: Vec<_> = [1, 2, 3]
            .into_iter()
            .skip_last()
            .skip_last()
            .skip_last()
            .collect();
        assert!(data3.is_empty());
    }

    #[test]
    fn test_skip_last_strings() {
        let strings = vec!["a", "b", "c", "d", "e"];
        let skipped_strings: Vec<_> = strings.into_iter().skip_last().collect();
        assert_eq!(skipped_strings, &["a", "b", "c", "d"]);
    }
}

#[cfg(test)]
mod tests_unique_elements {
    use super::*;

    // cargo test -- --help
    // cargo test -- --nocapture
    // cargo test -- --show-output
    // cargo test -- --show-output test_unique

    #[test]
    fn test_unique() {
        let mut vec = vec![1, 2, 2, 3, 1, 4, 3, 2, 5];
        vec.unique();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unique_empty() {
        let mut vec: Vec<i32> = vec![];
        vec.unique();
        assert_eq!(vec, Vec::<i32>::new());
        assert_eq!(vec, vec![] as Vec<i32>); // Alternative mode
    }

    #[test]
    fn test_unique_all_same() {
        let mut vec = vec![1, 1, 1, 1, 1];
        vec.unique();
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_unique_strings() {
        let mut vec = vec!["a", "b", "b", "c", "a", "d", "c", "b", "e"];
        vec.unique();
        assert_eq!(vec, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_unique_ordered() {
        let mut vec = vec![1, 2, 2, 3, 1, 4, 3, 2, 5];
        vec.unique_ordered();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unique_ordered_empty() {
        let mut vec: Vec<i32> = vec![];
        vec.unique_ordered();
        assert_eq!(vec, Vec::<i32>::new());
    }

    #[test]
    fn test_unique_ordered_all_same() {
        let mut vec = vec![1, 1, 1, 1, 1];
        vec.unique_ordered();
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_unique_ordered_strings() {
        let mut vec = vec!["a", "b", "b", "c", "a", "d", "c", "b", "e"];
        vec.unique_ordered();
        assert_eq!(vec, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_unique_mixed_types() {
        let mut vec: Vec<String> = vec![
            "a".to_string(),
            "b".to_string(),
            "b".to_string(),
            "c".to_string(),
            "a".to_string(),
        ];
        vec.unique();
        assert_eq!(vec, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn test_unique_ordered_mixed_types() {
        let mut vec: Vec<String> = vec![
            "a".to_string(),
            "b".to_string(),
            "b".to_string(),
            "c".to_string(),
            "a".to_string(),
        ];
        vec.unique_ordered();
        assert_eq!(vec, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn test_unique_numbers() {
        let mut vec = vec![5, 4, 3, 2, 1, 1, 2, 3, 4, 5];
        vec.unique();
        assert_eq!(vec, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_unique_ordered_numbers() {
        let mut vec = vec![5, 4, 3, 2, 1, 1, 2, 3, 4, 5];
        vec.unique_ordered();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unique_already_unique() {
        let mut vec = vec![1, 2, 3, 4, 5];
        vec.unique();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unique_ordered_already_unique() {
        let mut vec = vec![1, 2, 3, 4, 5];
        vec.unique_ordered();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unique_negative_numbers() {
        let mut vec = vec![-1, -2, -2, -3, -1, -4, -3, -2, -5];
        vec.unique();
        assert_eq!(vec, vec![-1, -2, -3, -4, -5]);
    }

    #[test]
    fn test_unique_ordered_negative_numbers() {
        let mut vec = vec![-1, -2, -2, -3, -1, -4, -3, -2, -5];
        vec.unique_ordered();
        assert_eq!(vec, vec![-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_unique_mixed_positive_negative() {
        let mut vec = vec![-1, 2, -2, 3, -1, 4, -3, 2, -5];
        vec.unique();
        assert_eq!(vec, vec![-1, 2, -2, 3, 4, -3, -5]);
    }

    #[test]
    fn test_unique_ordered_mixed_positive_negative() {
        let mut vec = vec![-1, 2, -2, 3, -1, 4, -3, 2, -5];
        vec.unique_ordered();
        assert_eq!(vec, vec![-5, -3, -2, -1, 2, 3, 4]);
    }

    // Test structs
    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    struct MyStruct {
        value: i32,
    }

    impl Ord for MyStruct {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.value.cmp(&other.value)
        }
    }

    impl PartialOrd for MyStruct {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[test]
    fn test_unique_structs() {
        let mut vec = vec![
            MyStruct { value: 1 },
            MyStruct { value: 2 },
            MyStruct { value: 2 },
            MyStruct { value: 3 },
            MyStruct { value: 1 },
        ];
        vec.unique();
        assert_eq!(
            vec,
            vec![
                MyStruct { value: 1 },
                MyStruct { value: 2 },
                MyStruct { value: 3 },
            ]
        );
    }

    #[test]
    fn test_unique_ordered_structs() {
        let mut vec = vec![
            MyStruct { value: 3 },
            MyStruct { value: 1 },
            MyStruct { value: 2 },
            MyStruct { value: 2 },
            MyStruct { value: 1 },
        ];
        vec.unique_ordered();
        assert_eq!(
            vec,
            vec![
                MyStruct { value: 1 },
                MyStruct { value: 2 },
                MyStruct { value: 3 },
            ]
        );
    }
}
