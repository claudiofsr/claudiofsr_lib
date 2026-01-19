/**
Trait extension for Slice

I'm looking for a nice way to split a slice into N chunks of approximately equal size.

The chunks method allows me to split a slice into chunks of size M, with the last chunk having the remainder,
but that results in the last one sometimes being much smaller.

I want all the chunks to be the same size or one greater.

Font: <https://users.rust-lang.org/t/how-to-split-a-slice-into-n-chunks/40008/6>.
*/
pub trait SliceExtension<T> {
    /**
    Returns an iterator over chunk_size elements of the slice at a time,
    starting at the beginning of the slice.

    When the length of the slice is not evenly divided by chunk_size,
    the first slices will have one more element, until to consume the remainders.

    The slice will be divided into N pieces such that N <= slice.len()

    ```
        use claudiofsr_lib::SliceExtension;

        // Example 1: an array with 5 items divided into 2 parts
        let data: [char; 5] = ['l', 'o', 'r', 'e', 'm'];
        let mut vector: Vec<&[char]> = Vec::new();

        data
            .chunks_at_most(2)
            .for_each(|v| {
                println!("{v:?}");
                vector.push(v);
            });

        assert_eq!(vector, vec![vec!['l', 'o', 'r'], vec!['e', 'm']]);
        assert_eq!(vector.len(), 2);

        // Example 2: an array with 4 items divided into 0, 1, .., 5 parts
        let data: [u16; 4] = [3, 67, 0, 9];
        let vector_0: Vec<&[u16]> = data.chunks_at_most(0).collect();
        let vector_1: Vec<&[u16]> = data.chunks_at_most(1).collect();
        let vector_2: Vec<&[u16]> = data.chunks_at_most(2).collect();
        let vector_3: Vec<&[u16]> = data.chunks_at_most(3).collect();
        let vector_4: Vec<&[u16]> = data.chunks_at_most(4).collect();
        let vector_5: Vec<&[u16]> = data.chunks_at_most(5).collect();

        assert!(vector_0.is_empty());
        assert_eq!(vector_1, [vec![3, 67, 0, 9]]);
        assert_eq!(vector_2, [vec![3, 67], vec![0, 9]]);
        assert_eq!(vector_3, [vec![3, 67], vec![0], vec![9]]);
        assert_eq!(vector_4, [&[3], &[67], &[0], &[9]]);
        assert_eq!(vector_5, vector_4);

        // Example 3: a vector with 25 elements divided into 4 parts
        let data: Vec<usize> = (1..=25).collect();
        let n_pieces = 4; // remainder: 25 % 4 = 1
        let pieces: Vec<&[usize]> = data.chunks_at_most(n_pieces).collect();
        // As a result, we will have:
        let result: Vec<&[usize]> = vec![
            &[ 1,  2,  3,  4,  5,  6,  7],
            &[ 8,  9, 10, 11, 12, 13],
            &[14, 15, 16, 17, 18, 19],
            &[20, 21, 22, 23, 24, 25],
        ];

        assert_eq!(result, pieces);
        assert_eq!(result.len(), 4);

        // Run the following test to see the results:
        // `cargo test -- --show-output divided_into_n_pieces`
    ```
    */
    fn chunks_at_most<'a>(&'a self, chunk_size: usize) -> impl Iterator<Item = &'a [T]>
    where
        T: 'a;
}

impl<T> SliceExtension<T> for [T] {
    fn chunks_at_most<'a>(&'a self, chunk_size: usize) -> impl Iterator<Item = &'a [T]>
    where
        T: 'a,
    {
        ChunksAtMost::new(self, chunk_size)
    }
}

// https://doc.rust-lang.org/src/core/slice/iter.rs.html#1436-1550

#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ChunksAtMost<'a, T: 'a> {
    data_slice: &'a [T],
    chunk_size: usize,
}

impl<'a, T: 'a> ChunksAtMost<'a, T> {
    #[inline]
    pub(super) fn new(slice: &'a [T], chunk_size: usize) -> Self {
        Self {
            data_slice: slice,
            chunk_size,
        }
    }
}

impl<T> Clone for ChunksAtMost<'_, T> {
    fn clone(&self) -> Self {
        ChunksAtMost {
            data_slice: self.data_slice,
            chunk_size: self.chunk_size,
        }
    }
}

impl<'a, T> Iterator for ChunksAtMost<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.chunk_size == 0 || self.data_slice.is_empty() {
            return None;
        }
        let group_number = (self.data_slice.len()).div_ceil(self.chunk_size);
        let (first, second) = self.data_slice.split_at(group_number);
        self.data_slice = second;
        self.chunk_size -= 1;
        Some(first)
    }
}

/// Print slice divided by n subsets
///
/// Run the following test to see the results:
///
/// `cargo test -- --show-output divided_into_n_pieces`
pub fn print_slice_divided_by_n_subsets<T>(data: &[T], n_pieces: usize) -> Vec<&[T]>
where
    T: std::fmt::Debug,
{
    let total = data.len();
    let size = total / n_pieces;
    let remainder = total % n_pieces;

    if remainder > 0 {
        println!(
            "total {total} divided into {n_pieces:2} pieces ; size: {size} or {} ; remainder: {remainder}",
            size + 1
        );
    } else {
        println!(
            "total {total} divided into {n_pieces:2} pieces ; size: {size} ; remainder: {remainder}"
        );
    }

    let vector: Vec<&[T]> = data.chunks_at_most(n_pieces).collect();
    let mut sum_of_all_pieces = 0;

    for pieces in &vector {
        sum_of_all_pieces += pieces.len();
        if pieces.len() < size || pieces.len() > size + 1 {
            eprintln!("pieces: {pieces:?} [{}]", pieces.len());
            panic!("Erro na função print_slice_divided_by_n_subsets()!")
        }
    }

    if total != sum_of_all_pieces {
        eprintln!("data: {data:?}");
        eprintln!("vector: {vector:?}");
        eprintln!("total: {total} != sum_of_all_pieces: {sum_of_all_pieces}");
        panic!("Erro na função print_slice_divided_by_n_subsets()!")
    }

    vector
}

#[cfg(test)]
mod slice_tests {
    // cargo test -- --help
    // cargo test -- --nocapture
    // cargo test -- --show-output
    use super::*;

    /// Split a slice into exactaly N pieces.
    ///
    /// `cargo test -- --show-output divided_into_n_pieces`
    #[test]
    fn divided_into_n_pieces() {
        let total = 25;
        let my_vec: Vec<usize> = (1..=total).collect();
        println!("my_vec: {my_vec:?}\n");

        for n_pieces in 1..=total {
            let vectors = print_slice_divided_by_n_subsets(&my_vec, n_pieces);
            println!("vectors: {vectors:?}");
            for (index, vector) in vectors.iter().enumerate() {
                let size = vector.len();
                println!(
                    "piece: {:2} ; size: {size:2} ; vector[{index:2}]: {vector:2?}",
                    index + 1
                );
            }
            println!();
        }

        println!("Extreme cases:");
        println!("1. attempt to divide slice by n_pieces such that n_pieces = 0.");
        let test_a: Vec<&[usize]> = my_vec.chunks_at_most(0).collect();

        println!("test_a: {test_a:?}");

        println!("2. attempt to divide slice by n_pieces such that n_pieces > slice.len().");
        let test_b: Vec<&[usize]> = my_vec.chunks_at_most(total + 1).collect();
        println!("test_b: {test_b:?}");

        let n_pieces = 4;
        let pieces: Vec<&[usize]> = my_vec.chunks_at_most(n_pieces).collect();
        let result: Vec<&[usize]> = vec![
            &[1, 2, 3, 4, 5, 6, 7],
            &[8, 9, 10, 11, 12, 13],
            &[14, 15, 16, 17, 18, 19],
            &[20, 21, 22, 23, 24, 25],
        ];

        assert_eq!(result, pieces);
    }
}
