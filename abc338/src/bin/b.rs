use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s
        .chars()
        .sorted_unstable()
        .dedup_with_count()
        .max_by_key(|&(cnt, c)| (cnt, Reverse(c)))
        .unwrap()
        .1;
    println!("{}", ans);
}
