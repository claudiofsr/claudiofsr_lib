/// Implement an Iterator with exactly N subsets of a slice.
///
/// When the length of the slice is not evenly divided by n_pieces,
/// the first slices will have one more element, until to consume the remainders.
///
/// ```
///     // Example: a vector with 25 elements divided into 4 parts
///     use claudiofsr_lib::split_slice_into_subsets;
///     let my_vec: Vec<usize> = (1..=25).collect();
///     let n_pieces = 4; // remainder: 25 % 4 = 1
///     let pieces: Vec<&[usize]> = split_slice_into_subsets(&my_vec, n_pieces).collect();
///     // As a result, we will have:
///     let result: Vec<&[usize]> = vec![
///         &[ 1,  2,  3,  4,  5,  6,  7],
///         &[ 8,  9, 10, 11, 12, 13],
///         &[14, 15, 16, 17, 18, 19],
///         &[20, 21, 22, 23, 24, 25],
///     ];
///     assert_eq!(result, pieces);
///
///     // Run the following test to see the results:
///     // `cargo test -- --show-output divided_into_n_pieces`
/// ```
pub fn split_slice_into_subsets<T>(data_slice: &[T], n_pieces: usize) -> impl Iterator<Item = &[T]>
//  where T: std::fmt::Debug,
{
    struct DataIter<'a, I> {
        data_slice: &'a [I],
        n_pieces: usize,
    }

    impl<'a, I> Iterator for DataIter<'a, I>
    //  where I: std::fmt::Debug,
    {
        type Item = &'a [I];
        fn next(&mut self) -> Option<&'a [I]> {
            if self.n_pieces == 0 || self.data_slice.is_empty() {
                return None;
            }
            let group_number = (self.data_slice.len()).div_ceil(self.n_pieces);
            let (first, second) = self.data_slice.split_at(group_number);
            self.data_slice = second;
            self.n_pieces -= 1;
            Some(first)
        }
    }

    DataIter { data_slice, n_pieces }
}

// cargo doc --open
// https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html
// https://github.com/rust-lang/rust/issues/63193

// Font:
// https://stackoverflow.com/questions/46867355/is-it-possible-to-split-a-vector-into-groups-of-10-with-iterators
// https://copyprogramming.com/howto/is-it-possible-to-split-a-vector-into-groups-of-10-with-iterators
// https://users.rust-lang.org/t/how-to-split-a-slice-into-n-chunks/40008/6
// https://geo-ant.github.io/blog/2022/implementing-parallel-iterators-rayon

/// Print slice divided by n subsets
pub fn print_slice_divided_by_n_subsets<T>(data: &[T], n_pieces: usize) -> Vec<&[T]>
    where T: std::fmt::Debug,
{
    let total = data.len();
    let size = total / n_pieces;
    let remainder = total % n_pieces;

    if remainder > 0 {
        println!("total {total} divided into {n_pieces:2} pieces ; size: {size} or {} ; remainder: {remainder}", size + 1);
    } else {
        println!("total {total} divided into {n_pieces:2} pieces ; size: {size} ; remainder: {remainder}");
    }

    let vector: Vec<&[T]> = split_slice_into_subsets(data, n_pieces).collect();
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
mod tests {
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

        for n_pieces in 1 ..= total {
            let vectors = print_slice_divided_by_n_subsets(&my_vec, n_pieces);
            println!("vectors: {vectors:?}");
            for (index, vector) in vectors.iter().enumerate() {
                let size = vector.len();
                println!("piece: {:2} ; size: {size:2} ; vector[{index:2}]: {vector:2?}", index + 1);
            }
            println!();
        }

        println!("Extreme cases:");
        println!("1. attempt to divide slice by n_pieces such that n_pieces = 0.");
        let test_a: Vec<&[usize]> = split_slice_into_subsets(&my_vec, 0).collect();
        println!("test_a: {test_a:?}");

        println!("2. attempt to divide slice by n_pieces such that n_pieces > slice.len().");
        let test_b: Vec<&[usize]> = split_slice_into_subsets(&my_vec, total + 1).collect();
        println!("test_b: {test_b:?}");

        let n_pieces = 4;
        let pieces: Vec<&[usize]> = split_slice_into_subsets(&my_vec, n_pieces).collect();
        let result: Vec<&[usize]> = vec![
            &[ 1,  2,  3,  4,  5,  6,  7],
            &[ 8,  9, 10, 11, 12, 13],
            &[14, 15, 16, 17, 18, 19],
            &[20, 21, 22, 23, 24, 25],
        ];

        assert_eq!(result, pieces);
    }
}