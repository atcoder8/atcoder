use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, k): (usize, usize),
        s: Chars,
    }

    let is_ok = |t: &[char]| {
        for left in 0..=n - k {
            if (0..k / 2).all(|j| t[left + j] == t[left + k - 1 - j]) {
                return false;
            }
        }

        true
    };

    let ans = s
        .into_iter()
        .permutations(n)
        .unique()
        .filter(|t| is_ok(t))
        .count();
    println!("{}", ans);
}
