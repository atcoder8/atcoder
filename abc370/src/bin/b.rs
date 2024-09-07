use ndarray::prelude::*;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n * (n + 1) / 2],
    }

    let mut transition = Array2::from_elem((n, n), n);
    let (mut row, mut col) = (0, 0);
    for &a in &aa {
        transition[(row, col)] = a;

        col += 1;
        if col > row {
            row += 1;
            col = 0;
        }
    }

    let mut cur = 0;
    for other in 0..n {
        if cur >= other {
            cur = transition[(cur, other)];
        } else {
            cur = transition[(other, cur)];
        }
    }

    println!("{}", cur + 1);
}
