//! # Slice Partitioning Utilities
//!
//! Provides extension traits and iterators for dividing slices into balanced subsets,
//! distributing any remainder elements evenly across the initial partitions.

use std::iter::FusedIterator;

/// Extension trait for slices providing balanced $N$-way partitioning.
///
/// Splits a slice into at most $N$ approximately equal-sized chunks, where earlier chunks
/// receive the remainder elements one by one until all items are distributed evenly.
///
/// Source: <https://users.rust-lang.org/t/how-to-split-a-slice-into-n-chunks/40008/6>
pub trait SliceExtension<T> {
    /**
    Returns an iterator yielding at most `n_chunks` contiguous, balanced sub-slices.

    When the length of the slice is not evenly divisible by `n_chunks`, the initial
    sub-slices will each have one extra element until all remainder items are consumed.

    # Behaviors and Constraints
    - If `n_chunks == 0` or the slice is empty, the iterator yields nothing.
    - If `n_chunks >= slice.len()`, each non-empty chunk contains exactly 1 element.
    - The returned iterator implements [`ExactSizeIterator`] and [`FusedIterator`].

    # Examples

    ```
    use claudiofsr_lib::SliceExtension;

    // Example 1: 5 elements split into at most 2 balanced parts
    let data: [char; 5] = ['l', 'o', 'r', 'e', 'm'];
    let vector: Vec<&[char]> = data.chunks_at_most(2).collect();

    assert_eq!(vector, vec![&['l', 'o', 'r'][..], &['e', 'm'][..]]);
    assert_eq!(vector, [&data[..3], &data[3..]]);
    assert_eq!(vector.len(), 2);

    // Example 2: 4 elements split across 0..=5 partitions
    let data: [u16; 4] = [3, 67, 0, 9];
    let vector_0: Vec<&[u16]> = data.chunks_at_most(0).collect();
    let vector_1: Vec<&[u16]> = data.chunks_at_most(1).collect();
    let vector_2: Vec<&[u16]> = data.chunks_at_most(2).collect();
    let vector_3: Vec<&[u16]> = data.chunks_at_most(3).collect();
    let vector_4: Vec<&[u16]> = data.chunks_at_most(4).collect();
    let vector_5: Vec<&[u16]> = data.chunks_at_most(5).collect();

    assert!(vector_0.is_empty());

    assert_eq!(vector_1, [&[3, 67, 0, 9][..]]);
    assert_eq!(vector_2, [&[3, 67][..], &[0, 9][..]]);
    assert_eq!(vector_3, [&[3, 67][..], &[0][..], &[9][..]]);
    assert_eq!(vector_4, [&[3][..], &[67][..], &[0][..], &[9][..]]);
    assert_eq!(vector_5, vector_4);

    assert_eq!(vector_1, [&data[..]]);
    assert_eq!(vector_2, [&data[..2], &data[2..]]);
    assert_eq!(vector_3, [&data[..2], &data[2..3], &data[3..]]);
    assert_eq!(vector_4, [&data[0..1], &data[1..2], &data[2..3], &data[3..4]]);
    assert_eq!(vector_5, vector_4);

    // Example 3: 25 elements split into 4 parts (25 % 4 = 1 remainder)
    let data: Vec<usize> = (1..=25).collect();
    let pieces: Vec<&[usize]> = data.chunks_at_most(4).collect();

    let expected: Vec<&[usize]> = vec![
        &[1, 2, 3, 4, 5, 6, 7],
        &[8, 9, 10, 11, 12, 13],
        &[14, 15, 16, 17, 18, 19],
        &[20, 21, 22, 23, 24, 25],
    ];

    assert_eq!(pieces, expected);
    ```
    */
    fn chunks_at_most<'a>(&'a self, n_chunks: usize) -> ChunksAtMost<'a, T>
    where
        T: 'a;
}

impl<T> SliceExtension<T> for [T] {
    #[inline]
    fn chunks_at_most<'a>(&'a self, n_chunks: usize) -> ChunksAtMost<'a, T>
    where
        T: 'a,
    {
        ChunksAtMost::new(self, n_chunks)
    }
}

/// An iterator yielding at most $N$ balanced, contiguous sub-slices.
///
/// Created by the [`chunks_at_most`](SliceExtension::chunks_at_most) method on slices.
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ChunksAtMost<'a, T: 'a> {
    data_slice: &'a [T],
    remaining_chunks: usize,
}

impl<'a, T: 'a> ChunksAtMost<'a, T> {
    /// Creates a new `ChunksAtMost` iterator over the provided slice.
    #[inline]
    pub fn new(slice: &'a [T], n_chunks: usize) -> Self {
        Self {
            data_slice: slice,
            remaining_chunks: n_chunks,
        }
    }
}

impl<'a, T> Iterator for ChunksAtMost<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Fast-path termination when no chunks remain or data is exhausted
        if self.remaining_chunks == 0 || self.data_slice.is_empty() {
            return None;
        }

        // Integer ceiling division computes the balanced chunk length: ceil(len / remaining)
        let chunk_len = self.data_slice.len().div_ceil(self.remaining_chunks);

        // Safe bounds splitting without panics
        let (head, tail) = self.data_slice.split_at_checked(chunk_len)?;

        self.data_slice = tail;
        self.remaining_chunks -= 1;

        Some(head)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.remaining_chunks.min(self.data_slice.len());
        (count, Some(count))
    }
}

impl<T> ExactSizeIterator for ChunksAtMost<'_, T> {}

impl<T> FusedIterator for ChunksAtMost<'_, T> {}

/// Diagnostic helper function to validate and display a slice partitioned into $N$ subsets.
///
/// # Panics
/// Panics if the generated chunk count or cumulative item total does not match expectations.
pub fn print_slice_divided_by_n_subsets<T>(data: &[T], n_pieces: usize) -> Vec<&[T]>
where
    T: std::fmt::Debug,
{
    if n_pieces == 0 {
        println!("Requested 0 pieces: returning an empty vector.");
        return Vec::new();
    }

    let total = data.len();
    let base_size = total / n_pieces;
    let remainder = total % n_pieces;

    if remainder > 0 {
        println!(
            "Total {total} divided into {n_pieces:2} pieces; size: {base_size} or {}; remainder: {remainder}",
            base_size + 1
        );
    } else {
        println!(
            "Total {total} divided into {n_pieces:2} pieces; size: {base_size}; remainder: {remainder}"
        );
    }

    let vector: Vec<&[T]> = data.chunks_at_most(n_pieces).collect();
    let sum_of_all_pieces: usize = vector.iter().map(|p| p.len()).sum();

    for piece in &vector {
        if piece.len() < base_size || piece.len() > base_size + 1 {
            eprintln!("Invalid chunk detected: {piece:?} (len: {})", piece.len());
            panic!(
                "Validation error in print_slice_divided_by_n_subsets: chunk size out of bounds!"
            );
        }
    }

    if total != sum_of_all_pieces {
        eprintln!("Total length mismatch: expected {total}, got {sum_of_all_pieces}");
        panic!(
            "Validation error in print_slice_divided_by_n_subsets: sum of pieces does not match total!"
        );
    }

    vector
}

// ==============================================================================
// SLICE TESTS
// ==============================================================================

#[cfg(test)]
mod slice_tests {
    // cargo test -- --help
    // cargo test -- --nocapture
    // cargo test -- --show-output
    use super::*;

    /// Split a slice into exactaly N pieces.
    ///
    /// cargo test -- --show-output divided_into_n_pieces
    #[test]
    fn divided_into_n_pieces() {
        let total = 25;
        let my_vec: Vec<usize> = (1..=total).collect();
        println!("my_vec: {my_vec:?}\n");

        // ---------------------------------------------------------------------
        // Part 1: Comprehensive partitioning loop test from 1 to total elements
        // ---------------------------------------------------------------------
        for n_pieces in 1..=total {
            let vectors = print_slice_divided_by_n_subsets(&my_vec, n_pieces);
            println!("vectors: {vectors:?}");

            // 1. Verify that the number of generated partitions exactly matches n_pieces
            assert_eq!(vectors.len(), n_pieces);

            let base_size = total / n_pieces;

            for (index, vector) in vectors.iter().enumerate() {
                let size = vector.len();
                println!(
                    "piece: {:2} ; size: {size:2} ; vector[{index:2}]: {vector:2?}",
                    index + 1
                );

                // 2. Verify that each chunk length is either base_size or (base_size + 1)
                //    This validates the balanced partitioning invariant.
                assert!(
                    size == base_size || size == base_size + 1,
                    "Invalid chunk size at piece {}: got {}, expected {} or {}",
                    index + 1,
                    size,
                    base_size,
                    base_size + 1
                );
            }

            // 3. Fast contiguous slice concatenation: ensures all elements are
            //    preserved in order without data loss or duplication.
            assert_eq!(vectors.concat(), my_vec);

            println!();
        }

        // ---------------------------------------------------------------------
        // Part 2: Boundary and extreme edge cases
        // ---------------------------------------------------------------------
        println!("Extreme cases:");

        // Case A: Requesting 0 partitions on a non-empty slice
        println!("1. attempt to divide slice by n_pieces such that n_pieces = 0.");
        let test_a: Vec<&[usize]> = my_vec.chunks_at_most(0).collect();
        println!("test_a: {test_a:?}");

        assert!(
            test_a.is_empty(),
            "test_a should be empty when n_pieces == 0"
        );
        assert_eq!(test_a.len(), 0);

        // Case B: Requesting more partitions than available elements (e.g. 26 chunks for 25 items)
        println!("2. attempt to divide slice by n_pieces such that n_pieces > slice.len().");
        let test_b: Vec<&[usize]> = my_vec.chunks_at_most(total + 1).collect();
        println!("test_b: {test_b:?}");

        // The chunk count must be naturally capped at total elements (each of length 1)
        assert_eq!(
            test_b.len(),
            total,
            "test_b chunk count should be capped at total elements"
        );
        assert!(
            test_b.iter().all(|chunk| chunk.len() == 1),
            "Every chunk in test_b must contain exactly 1 element"
        );
        assert_eq!(test_b.concat(), my_vec);

        // ---------------------------------------------------------------------
        // Part 3: Explicit structural validation for a known remainder
        // ---------------------------------------------------------------------
        let n_pieces = 4;
        let pieces: Vec<&[usize]> = my_vec.chunks_at_most(n_pieces).collect();

        // 25 / 4 = 6 with remainder 1 -> first chunk has 7 items, remaining 3 chunks have 6 items
        let expected: Vec<&[usize]> = vec![
            &[1, 2, 3, 4, 5, 6, 7],
            &[8, 9, 10, 11, 12, 13],
            &[14, 15, 16, 17, 18, 19],
            &[20, 21, 22, 23, 24, 25],
        ];

        assert_eq!(pieces, expected);
        assert_eq!(pieces.concat(), my_vec);
    }

    #[test]
    fn test_exact_size_iterator_and_len() {
        let data = [10, 20, 30, 40, 50];
        let mut iter = data.chunks_at_most(3);

        assert_eq!(iter.len(), 3);
        let first = iter.next();
        assert_eq!(first, Some(&[10, 20][..]));
        assert_eq!(iter.len(), 2);

        let second = iter.next();
        assert_eq!(second, Some(&[30, 40][..]));
        assert_eq!(iter.len(), 1);

        let third = iter.next();
        assert_eq!(third, Some(&[50][..]));
        assert_eq!(iter.len(), 0);

        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
    }
}
