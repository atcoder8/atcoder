use std::collections::VecDeque;

use itertools::iproduct;
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
    }

    let find_cell =
        |symbol: char| iproduct!(0..h, 0..w).find(|(row, col)| aaa[*row][*col] == symbol);

    let start_cell = find_cell('S').unwrap();
    let goal_cell = find_cell('G').unwrap();

    let passable = |cell: (usize, usize), flipped: bool| {
        let (row, col) = cell;
        let symbol = aaa[row][col];
        let blocked = symbol == '#' || (symbol == 'o' && flipped) || (symbol == 'x' && !flipped);
        !blocked
    };

    let mut dist_array = Array2::from_elem((h, w), [None::<usize>; 2]);
    let mut queue = VecDeque::from([(start_cell, 0, false)]);
    while let Some((cell, cand_dist, flipped)) = queue.pop_front() {
        let dist = &mut dist_array[cell][flipped as usize];

        if !passable(cell, flipped) || dist.is_some() {
            continue;
        }

        *dist = Some(cand_dist);

        let (row, col) = cell;
        for (diff_row, diff_col) in DIFFS {
            let adj_row = row.wrapping_add(diff_row);
            let adj_col = col.wrapping_add(diff_col);
            let adj_coord = (adj_row, adj_col);
            if adj_row < h && adj_col < w {
                queue.push_back((
                    adj_coord,
                    cand_dist + 1,
                    flipped ^ (aaa[adj_row][adj_col] == '?'),
                ));
            }
        }
    }

    let min_dist = dist_array[(goal_cell.0, goal_cell.1)]
        .iter()
        .filter_map(|&dist| dist)
        .min();
    match min_dist {
        Some(dist) => println!("{dist}"),
        None => println!("-1"),
    }
}
