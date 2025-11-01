use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let grid = Array2::from_shape_fn((n, n), |(row, col)| ss[row][col]);
    let count = iproduct!(0..=n - m, 0..=n - m)
        .map(|(top, left)| grid.slice(s![top..top + m, left..left + m]))
        .unique()
        .count();
    println!("{count}");
}
