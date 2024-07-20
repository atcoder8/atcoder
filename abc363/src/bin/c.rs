use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, k): (usize, usize),
        mut s: Chars,
    }

    let is_ok = |t: &[char]| {
        for left in 0..=n - k {
            if (0..k / 2).all(|j| t[left + j] == t[left + k - 1 - j]) {
                return false;
            }
        }

        true
    };

    s.sort_unstable();

    let mut ans = 0;
    loop {
        ans += is_ok(&s) as usize;

        if !next_permutation(&mut s) {
            break;
        }
    }
    println!("{}", ans);
}

/// If there is a next permutation of `seq` with respect to the lexicographic order, replace `seq` with it (return value is `true`).
/// Otherwise (i.e., if `seq` is already in descending order), it reverts to ascending order (return value is `false`).
pub fn next_permutation<T>(seq: &mut [T]) -> bool
where
    T: Ord,
{
    // If `seq` is empty, `false` is returned because there is no next permutation.
    if seq.is_empty() {
        return false;
    }

    // Find the maximum value of `i` such that `seq[i] < seq[i + 1]`.
    // If no such `i` exists, `seq` has already been sorted in descending order,
    // so return `false` after sorting in ascending order.
    let Some(i) = (0..seq.len() - 1).rev().find(|&i| seq[i] < seq[i + 1]) else {
        seq.reverse();
        return false;
    };

    // Find the largest `j` that satisfies `j > i` and `seq[j] > seq[i]`, and exchange `seq[i]` and `seq[j]`.
    // Because `seq[i] < seq[i + 1]`, there is always such `j`.
    let j = (i + 1..seq.len()).rev().find(|&j| seq[j] > seq[i]).unwrap();
    seq.swap(i, j);

    // Sort elements after the `i`-th in ascending order to minimize the increase with respect to lexicographic order.
    seq[i + 1..].reverse();

    true
}
