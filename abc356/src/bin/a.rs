use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, l, r): (usize, Usize1, usize),
    }

    let mut seq = (1..=n).collect_vec();
    seq[l..r].reverse();

    println!("{}", seq.iter().join(" "));
}
