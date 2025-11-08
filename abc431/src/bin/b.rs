use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        x: u64,
        n: usize,
        ww: [u64; n],
        q: usize,
        pp: [Usize1; q],
    }

    let mut weight = x;
    let mut belongs = vec![false; n];
    let output = pp
        .iter()
        .map(|&p| {
            if belongs[p] {
                weight -= ww[p];
            } else {
                weight += ww[p];
            }
            belongs[p] = !belongs[p];

            weight
        })
        .join("\n");
    println!("{output}");
}
