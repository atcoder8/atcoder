use std::collections::BinaryHeap;

use itertools::iproduct;
use ndarray::{Array2, Array3};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
        n: usize,
        rce: [(Usize1, Usize1, usize); n],
    }

    let start = iproduct!(0..h, 0..w)
        .find(|&(row, col)| aaa[row][col] == 'S')
        .unwrap();
    let goal = iproduct!(0..h, 0..w)
        .find(|&(row, col)| aaa[row][col] == 'T')
        .unwrap();

    let mut medicine_mat = Array2::from_elem((h, w), 0_usize);
    for &(r, c, e) in &rce {
        medicine_mat[(r, c)] = e;
    }

    let mut dp = Array3::<Option<usize>>::from_elem((h, w, 2), None);

    let mut heap = BinaryHeap::from([(0, start, false), (medicine_mat[start], start, true)]);

    while let Some((health, (row, col), used)) = heap.pop() {
        if dp[(row, col, used as usize)].is_some_and(|prev_health| health <= prev_health) {
            continue;
        }

        dp[(row, col, used as usize)] = Some(health);

        if health == 0 {
            continue;
        }

        for (diff_row, diff_col) in DIFFS {
            let next_row = row.wrapping_add(diff_row);
            let next_col = col.wrapping_add(diff_col);

            if next_row < h && next_col < w && aaa[next_row][next_col] != '#' {
                heap.push((health - 1, (next_row, next_col), false));
                heap.push((
                    medicine_mat[(next_row, next_col)],
                    (next_row, next_col),
                    true,
                ));
            }
        }
    }

    let ans = dp[(goal.0, goal.1, 0)].is_some() || dp[(goal.0, goal.1, 1)].is_some();
    println!("{}", if ans { "Yes" } else { "No" });
}
