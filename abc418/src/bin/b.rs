use std::ops::Range;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let calc_rate = |range: Range<usize>| {
        let substring = &s[range];

        if substring.len() < 3 || substring[0] != 't' || substring[substring.len() - 1] != 't' {
            return 0.0;
        }

        let num_ts = substring.iter().filter(|&&ch| ch == 't').count();
        (num_ts - 2) as f64 / (substring.len() - 2) as f64
    };

    let max_rate = (0..=s.len())
        .tuple_combinations()
        .map(|(left, right)| calc_rate(left..right))
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap_or(0.0);
    println!("{max_rate}");
}
