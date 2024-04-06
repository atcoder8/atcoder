use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let solve = |coord: (i64, i64)| {
        xy.iter()
            .enumerate()
            .position_max_by_key(|&(i, other)| {
                (
                    (coord.0 - other.0).pow(2) + (coord.1 - other.1).pow(2),
                    Reverse(i),
                )
            })
            .unwrap()
            + 1
    };

    let ans = xy.iter().map(|&coord| solve(coord)).join("\n");
    println!("{}", ans);
}
