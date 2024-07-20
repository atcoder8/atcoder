use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use ndarray::Array2;
use proconio::input;

const MAX: usize = 10_usize.pow(5);

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w, y): (usize, usize, usize),
        aaa: [[usize; w]; h],
    }

    let mut mat = Array2::from_elem((h, w), MAX + 1);

    let mut surround = vec![];
    surround
        .extend((0..h).flat_map(|row| [(aaa[row][0], (row, 0)), (aaa[row][w - 1], (row, w - 1))]));
    surround
        .extend((0..w).flat_map(|col| [(aaa[0][col], (0, col)), (aaa[h - 1][col], (h - 1, col))]));

    let mut heap: BinaryHeap<_> = surround.into_iter().map(Reverse).collect();
    while let Some(Reverse((height, coord))) = heap.pop() {
        if height >= mat[coord] {
            continue;
        }

        mat[coord] = height;

        let (row, col) = coord;
        for (diff_row, diff_col) in DIFFS {
            let next_row = row.wrapping_add(diff_row);
            let next_col = col.wrapping_add(diff_col);
            let next_coord = (next_row, next_col);

            if next_row < h && next_col < w {
                heap.push(Reverse((height.max(aaa[next_row][next_col]), next_coord)));
            }
        }
    }

    let mut counts = vec![0; y + 1];
    for &height in mat.iter() {
        if height <= y {
            counts[height] += 1;
        }
    }
    for i in 0..y {
        counts[i + 1] += counts[i];
    }

    let ans = (1..=y).map(|y| h * w - counts[y]).join("\n");
    println!("{}", ans);
}
