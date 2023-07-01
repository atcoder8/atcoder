use itertools::{join, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let order = (0..n).sorted_by(|&i, &j| {
        let (a1, b1) = ab[i];
        let (a2, b2) = ab[j];

        (a1 * (a2 + b2)).cmp(&(a2 * (a1 + b1))).reverse()
    });

    println!("{}", join(order.map(|i| i + 1), " "));
}
