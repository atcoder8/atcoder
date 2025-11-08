use std::collections::VecDeque;

use itertools::Itertools;
use ndarray::Array2;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(0, 1), (!0, 0), (0, !0), (1, 0)];

const DIRECTION_MAP: [[usize; 3]; 4] = [[0, 3, 1], [1, 2, 0], [2, 1, 3], [3, 0, 2]];

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> usize {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let mut cost_array = Array2::from_elem((h, w), [None::<usize>; 4]);

    let mut queue = VecDeque::<Node>::from([Node::new([0; 2], 0, 0)]);
    let mut min_cost = None::<usize>;
    while let Some(node) = queue.pop_front() {
        if cost_array[node.coord][node.in_dir].is_some() {
            continue;
        }

        cost_array[node.coord][node.in_dir] = Some(node.cost);

        let [row, col] = node.coord;
        let init_mirror_type = (ss[row][col] as u8 - b'A') as usize;
        for mirror_type in 0..3 {
            let out_dir = DIRECTION_MAP[node.in_dir][mirror_type];
            let (dr, dc) = DIFFS[out_dir];
            let next_row = row.wrapping_add(dr);
            let next_col = col.wrapping_add(dc);

            if next_row == h - 1 && next_col == w {
                chmin_for_option(
                    &mut min_cost,
                    node.cost + (mirror_type != init_mirror_type) as usize,
                );
            }

            if next_row >= h || next_col >= w {
                continue;
            }

            if mirror_type == init_mirror_type {
                queue.push_front(Node::new([next_row, next_col], out_dir, node.cost));
            } else {
                queue.push_back(Node::new([next_row, next_col], out_dir, node.cost + 1));
            }
        }
    }

    min_cost.unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Node {
    coord: [usize; 2],
    in_dir: usize,
    cost: usize,
}

impl Node {
    fn new(coord: [usize; 2], direction: usize, cost: usize) -> Self {
        Self {
            coord,
            in_dir: direction,
            cost,
        }
    }
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
