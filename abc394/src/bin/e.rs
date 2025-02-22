use std::collections::VecDeque;

use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ccc: [Chars; n],
    }

    let adjacent_matrix = Array2::from_shape_fn((n, n), |(row, col)| {
        if ccc[row][col] == '-' {
            None
        } else {
            Some(ccc[row][col])
        }
    });

    let mut min_lengths = Array2::from_elem((n, n), None::<usize>);

    for start in 0..n {
        let mut queue = VecDeque::<(usize, (usize, usize))>::from([(0, (start, start))]);
        while let Some((cand_cost, (from, to))) = queue.pop_front() {
            if min_lengths[(from, to)].is_some_and(|cost| cost <= cand_cost) {
                continue;
            }

            min_lengths[(from, to)] = Some(cand_cost);

            for c in 'a'..='z' {
                let next_from = (0..n).filter(|&adj| adjacent_matrix[(adj, from)] == Some(c));
                let next_to = (0..n).filter(|&adj| adjacent_matrix[(to, adj)] == Some(c));
                queue.extend(
                    iproduct!(next_from, next_to).map(|next_pair| (cand_cost + 2, next_pair)),
                );
            }
        }
    }

    for init_pair in iproduct!(0..n, 0..n) {
        if adjacent_matrix[init_pair].is_none() {
            continue;
        }

        let mut queue = VecDeque::<(usize, (usize, usize))>::from([(1, init_pair)]);
        while let Some((cand_cost, (from, to))) = queue.pop_front() {
            if min_lengths[(from, to)].is_some_and(|cost| cost <= cand_cost) {
                continue;
            }

            min_lengths[(from, to)] = Some(cand_cost);

            for c in 'a'..='z' {
                let next_from = (0..n).filter(|&adj| adjacent_matrix[(adj, from)] == Some(c));
                let next_to = (0..n).filter(|&adj| adjacent_matrix[(to, adj)] == Some(c));
                queue.extend(
                    iproduct!(next_from, next_to).map(|next_pair| (cand_cost + 2, next_pair)),
                );
            }
        }
    }

    let length_to_str = |length: Option<usize>| match length {
        Some(length) => length.to_string(),
        None => "-1".to_string(),
    };

    println!(
        "{}",
        min_lengths
            .axis_iter(Axis(0))
            .map(|line| line.iter().map(|&length| length_to_str(length)).join(" "))
            .join("\n")
    );
}

pub mod rolling_hash {
    //! Module for rolling hash.

    /// The type of the blocks that make up the hash.
    pub type HashBlock = u64;

    /// Number of integers that make up the hash value.
    pub const HASH_BLOCK_NUM: usize = 5;

    /// Type of hash value.
    ///
    /// A hash value consists of several integers.
    pub type HashValue = [HashBlock; HASH_BLOCK_NUM];

    /// Moduli used to calculate hash values.
    pub const MODULI: HashValue = [1000002637, 1000011659, 1000012631, 1000017841, 1000018603];

    /// Radixes used to calculate hash values.
    pub const RADIXES: HashValue = [252895580, 406082094, 892791812, 869052263, 261298120];

    /// Generates a hash value from a sequence using Rabin-Karp algorithm.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RollingHash {
        /// Length of the sequence.
        len: usize,

        /// Hash value corresponding to the sequence.
        hash_value: HashValue,

        /// Sequence length power of the radix.
        /// This array is used to calculate the hash value corresponding to the concatenated sequence.
        raised_radixes: HashValue,
    }

    impl Default for RollingHash {
        fn default() -> Self {
            Self {
                len: 0,
                hash_value: [0; HASH_BLOCK_NUM],
                raised_radixes: [1; HASH_BLOCK_NUM],
            }
        }
    }

    impl<T, I> From<I> for RollingHash
    where
        HashBlock: From<T>,
        I: IntoIterator<Item = T>,
    {
        /// Generates a hash value from a sequence.
        fn from(seq: I) -> Self {
            let mut hash = RollingHash::new();
            hash.extend(seq);

            hash
        }
    }

    impl RollingHash {
        /// Generates a hash value corresponding to an empty sequence.
        pub fn new() -> Self {
            Self {
                len: 0,
                raised_radixes: [1; HASH_BLOCK_NUM],
                hash_value: [0; HASH_BLOCK_NUM],
            }
        }

        /// Generates a hash value from a slice of the sequence.
        pub fn from_slice<T>(seq: &[T]) -> Self
        where
            HashBlock: From<T>,
            T: Copy,
        {
            Self::from(seq.iter().cloned())
        }

        /// Generates a hash value from a string slice.
        #[allow(clippy::should_implement_trait)]
        pub fn from_str(s: &str) -> Self {
            Self::from(s.chars())
        }

        /// Generates a hash value from a slice with elements of type `usize`.
        pub fn from_usize_slice(seq: &[usize]) -> Self {
            Self::from(seq.iter().map(|&elem| elem as HashBlock))
        }

        /// Generates a hash value from a sequence with elements of type `usize`.
        pub fn from_usize<I>(seq: I) -> Self
        where
            I: IntoIterator<Item = usize>,
        {
            Self::from(seq.into_iter().map(|elem| elem as HashBlock))
        }

        /// Returns the length of the sequence.
        pub fn len(&self) -> usize {
            self.len
        }

        /// Returns whether the sequence is empty or not.
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// Adds an element to the end of the sequence.
        pub fn push<T>(&mut self, elem: T)
        where
            HashBlock: From<T>,
        {
            self.len += 1;

            let elem = HashBlock::from(elem);
            for block_idx in 0..HASH_BLOCK_NUM {
                let radix = RADIXES[block_idx];
                let modulus = MODULI[block_idx];

                let block = &mut self.hash_value[block_idx];
                *block = (*block * radix % modulus + elem % modulus) % modulus;

                let raised_radix = &mut self.raised_radixes[block_idx];
                *raised_radix = *raised_radix * radix % modulus;
            }
        }

        /// Adds some elements to the end of the sequence.
        pub fn extend<T, I>(&mut self, elements: I)
        where
            HashBlock: From<T>,
            I: IntoIterator<Item = T>,
        {
            elements.into_iter().for_each(|elem| self.push(elem));
        }

        /// Concatenates another sequence behind the sequence.
        pub fn concat(&mut self, other: &RollingHash) {
            self.len += other.len;

            for (block_idx, modulus) in MODULI.iter().enumerate() {
                let block = &mut self.hash_value[block_idx];
                *block = (*block * other.raised_radixes[block_idx] % modulus
                    + other.hash_value[block_idx])
                    % modulus;

                let raised_radix = &mut self.raised_radixes[block_idx];
                *raised_radix = *raised_radix * other.raised_radixes[block_idx] % modulus;
            }
        }

        /// Generates a hash value from a chained sequence.
        pub fn chain(&self, other: &RollingHash) -> Self {
            let mut concatenated_hash = *self;
            concatenated_hash.concat(other);

            concatenated_hash
        }
    }
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
