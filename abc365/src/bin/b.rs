use std::cmp::Reverse;

use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let ans = enumerate(aa)
        .sorted_by_key(|&(_, a)| Reverse(a))
        .nth(1)
        .unwrap()
        .0
        + 1;
    println!("{}", ans);
}
