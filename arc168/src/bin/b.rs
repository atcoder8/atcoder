use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        aa: [usize; n],
    }

    if aa.iter().fold(0, |acc, &x| acc ^ x) != 0 {
        return None;
    }

    let max_odd = aa
        .into_iter()
        .sorted_unstable_by_key(|&a| Reverse(a))
        .dedup_with_count()
        .filter(|&(cnt, _)| cnt % 2 == 1)
        .next()
        .unzip()
        .1;

    match max_odd {
        Some(max_odd) => Some(max_odd - 1),
        None => Some(0),
    }
}
