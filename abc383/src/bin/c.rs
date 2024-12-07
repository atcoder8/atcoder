use std::collections::BinaryHeap;

use itertools::iproduct;
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w, d): (usize, usize, usize),
        sss: [Chars; h],
    }

    let mut heap: BinaryHeap<(usize, (usize, usize))> = iproduct!(0..h, 0..w)
        .filter_map(|coord| {
            if sss[coord.0][coord.1] == 'H' {
                Some((d + 1, coord))
            } else {
                None
            }
        })
        .collect();

    let mut grid = Array2::from_elem((h, w), 0_usize);
    while let Some((depth, coord)) = heap.pop() {
        if depth <= grid[coord] {
            continue;
        }

        grid[coord] = depth;

        if depth <= 1 {
            continue;
        }

        let (row, col) = coord;
        for (diff_row, diff_col) in DIFFS {
            let adj_row = row.wrapping_add(diff_row);
            let adj_col = col.wrapping_add(diff_col);
            let adj_coord = (adj_row, adj_col);

            if adj_row < h && adj_col < w && sss[adj_row][adj_col] != '#' {
                heap.push((depth - 1, adj_coord));
            }
        }
    }

    let ans = grid.iter().filter(|&&depth| depth != 0).count();
    println!("{}", ans);
}
