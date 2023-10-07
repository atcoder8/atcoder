use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let win_counts = ss
        .iter()
        .map(|s| s.iter().filter(|&&c| c == 'o').count())
        .collect_vec();
    let orders = (0..n).sorted_by_key(|&i| Reverse(win_counts[i]));
    println!("{}", orders.map(|order| order + 1).join(" "));
}
