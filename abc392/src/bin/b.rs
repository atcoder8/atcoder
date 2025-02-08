use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [Usize1; m],
    }

    let mut contains = vec![false; n];
    for &a in &aa {
        contains[a] = true;
    }

    let filtered = (0..n)
        .filter(|&i| !contains[i])
        .map(|i| i + 1)
        .collect_vec();
    println!("{}\n{}", filtered.len(), filtered.iter().join(" "));
}
