use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let num_all_pairs = m * (m - 1) / 2;
    let num_parallel_pairs = ab
        .iter()
        .map(|&(a, b)| (a + b) % n)
        .sorted_unstable()
        .dedup_with_count()
        .map(|(cnt, _)| cnt * (cnt - 1) / 2)
        .sum::<usize>();
    num_all_pairs - num_parallel_pairs
}
