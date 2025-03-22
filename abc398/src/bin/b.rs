use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        aa: [Usize1; 7],
    }

    let mut counts = [0_u8; 13];
    for &a in &aa {
        counts[a] += 1;
    }

    let ans = counts
        .iter()
        .permutations(2)
        .any(|perm| *perm[0] >= 3 && *perm[1] >= 2);
    println!("{}", if ans { "Yes" } else { "No" });
}
