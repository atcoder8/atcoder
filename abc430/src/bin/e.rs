use std::borrow::Cow;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| match solve() {
            Some(cost) => Cow::Owned(cost.to_string()),
            None => Cow::Borrowed("-1"),
        })
        .join("\n");
    println!("{output}");
}

fn solve() -> Option<usize> {
    input! {
        a: String,
        b: String,
    }

    let n = a.len();

    let concat = (b + &a + &a).chars().collect_vec();
    let lengths = z_algorithm(&concat);
    lengths[n..].iter().position(|&length| length >= n)
}

/// For each non-negative integer `i` less than `|seq|`,
/// find the length of the longest common prefix of `seq` and `seq[i..]`.
pub fn z_algorithm<T>(seq: &[T]) -> Vec<usize>
where
    T: Eq,
{
    if seq.is_empty() {
        return vec![];
    }

    let n = seq.len();

    let mut lengths = vec![0; n];
    lengths[0] = n;

    let mut cursor = 1;
    let mut common_len = 0;
    while cursor < n {
        while cursor + common_len < n && seq[cursor + common_len] == seq[common_len] {
            common_len += 1;
        }

        if common_len == 0 {
            cursor += 1;
            continue;
        }

        lengths[cursor] = common_len;

        let mut shift = 1;
        while shift + lengths[shift] < common_len {
            lengths[cursor + shift] = lengths[shift];
            shift += 1;
        }

        cursor += shift;
        common_len -= shift;
    }

    lengths
}
