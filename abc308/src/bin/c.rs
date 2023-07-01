use itertools::{join, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut abi = ab
        .iter()
        .enumerate()
        .map(|(i, &(a, b))| (a, b, i))
        .collect_vec();
    abi.sort_by(|&x, &y| {
        let (a1, b1, _) = x;
        let (a2, b2, _) = y;

        (a1 * (a2 + b2)).cmp(&(a2 * (a1 + b1))).reverse()
    });

    println!("{}", join(abi.iter().map(|&(_, _, i)| i + 1), " "));
}
