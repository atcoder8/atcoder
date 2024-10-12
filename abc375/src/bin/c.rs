use itertools::Itertools;
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        grid: [Chars; n],
    }

    let mut rotated_grid = Array2::from_elem((n, n), '*');
    for i in 1..=n / 2 {
        // i mod 4 回転
        let calc_rotated_coord = |coord: (usize, usize)| {
            let (x, y) = coord;
            [
                (x, y),
                (y, n - 1 - x),
                (n - 1 - x, n - 1 - y),
                (n - 1 - y, x),
            ][i % 4]
        };

        for j in i..=n + 1 - i {
            rotated_grid[calc_rotated_coord((j - 1, i - 1))] = grid[j - 1][i - 1];
            rotated_grid[calc_rotated_coord((j - 1, n - i))] = grid[j - 1][n - i];
            rotated_grid[calc_rotated_coord((i - 1, j - 1))] = grid[i - 1][j - 1];
            rotated_grid[calc_rotated_coord((n - i, j - 1))] = grid[n - i][j - 1];
        }
    }

    let ans = rotated_grid
        .axis_iter(Axis(0))
        .map(|line| line.iter().join(""))
        .join("\n");
    println!("{}", ans);
}
