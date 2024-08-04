// unfinished

use itertools::Itertools;
use ndarray::Array2;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
        qq: [Usize1; n],
    }

    let matrix = Array2::from_shape_fn((n, n), |(row, col)| (col >= n - 1 - row) as u8);
    let ans = pp
        .iter()
        .map(|&p| qq.iter().map(|&q| matrix[(p, q)]).join(""))
        .join("\n");
    println!("{}", ans);
}
