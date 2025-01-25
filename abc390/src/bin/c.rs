use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let grid = Array2::from_shape_fn((h, w), |(row, col)| match ss[row][col] {
        '#' => Some(true),
        '.' => Some(false),
        '?' => None,
        _ => panic!(),
    });

    let is_black = |coord: (usize, usize)| grid[coord] == Some(true);

    let (mut top, mut bottom, mut left, mut right) = (h, 0_usize, w, 0_usize);
    for coord in iproduct!(0..h, 0..w) {
        if !is_black(coord) {
            continue;
        }

        let (row, col) = coord;
        top = top.min(row);
        bottom = bottom.max(row + 1);
        left = left.min(col);
        right = right.max(col + 1);
    }

    let ans = iproduct!(top..bottom, left..right).all(|coord| grid[coord].unwrap_or(true));
    println!("{}", if ans { "Yes" } else { "No" });
}
