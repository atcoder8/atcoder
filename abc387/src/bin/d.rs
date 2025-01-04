use std::collections::VecDeque;

use itertools::{enumerate, iproduct};
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let find_coord = |c: char| {
        iproduct!(0..h, 0..w)
            .find(|&(row, col)| ss[row][col] == c)
            .unwrap()
    };

    let start_coord = find_coord('S');
    let goal_coord = find_coord('G');

    let mut visited = Array2::from_elem((h, w), [false; 4]);
    let mut queue: VecDeque<((usize, usize), usize, usize)> =
        (0..4).map(|dir| (start_coord, 0, dir)).collect();
    while let Some((coord, dist, dir)) = queue.pop_front() {
        if visited[coord][dir] {
            continue;
        }

        visited[coord][dir] = true;

        if coord == goal_coord {
            return Some(dist);
        }

        let (row, col) = coord;
        for (next_dir, (diff_row, diff_col)) in enumerate(DIFFS) {
            if dir / 2 == next_dir / 2 {
                continue;
            }

            let adj_row = row.wrapping_add(diff_row);
            let adj_col = col.wrapping_add(diff_col);
            let adj_coord = (adj_row, adj_col);

            if adj_row < h && adj_col < w && ss[adj_row][adj_col] != '#' {
                queue.push_back((adj_coord, dist + 1, next_dir));
            }
        }
    }

    None
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
