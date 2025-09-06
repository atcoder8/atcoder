use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (n, q): (usize, usize),
        sss: [Chars; n],
        udlr: [((Usize1, usize), (Usize1, usize)); q],
    }

    let mut imos = Array2::from_elem([n + 1; 2], 0);
    for (row, col) in iproduct!(0..n - 1, 0..n - 1) {
        imos[(row + 1, col + 1)] =
            iproduct!(0..2, 0..2).all(|(dr, dc)| sss[row + dr][col + dc] == '.') as usize;
    }
    for (row, col) in iproduct!(0..=n, 0..n) {
        imos[(row, col + 1)] += imos[(row, col)];
    }
    for (row, col) in iproduct!(0..n, 0..=n) {
        imos[(row + 1, col)] += imos[(row, col)];
    }

    let calc_rect_sum = |top: usize, bottom: usize, left: usize, right: usize| {
        imos[(bottom, right)] + imos[(top, left)] - (imos[(bottom, left)] + imos[(top, right)])
    };

    let output = udlr
        .iter()
        .map(|&((u, d), (l, r))| calc_rect_sum(u, d - 1, l, r - 1))
        .join("\n");
    println!("{output}");
}
