// unfinished

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        nm: [(usize, usize); t],
    }

    println!("{}", nm.iter().map(|&(n, m)| solve(n, m)).join("\n"));
}

fn solve(n: usize, m: usize) -> usize {
    if m == 1 {
        1
    } else {
        n + 1
    }
}
