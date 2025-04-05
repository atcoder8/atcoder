use std::collections::VecDeque;

use ndarray::prelude::*;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
        start: (Usize1, Usize1),
        goal: (Usize1, Usize1),
    }

    let mut cost_mat = Array2::from_elem((h, w), None::<usize>);

    let calc_adj_coord = |coord: (usize, usize), relative_coord: (usize, usize)| {
        let adj_row = coord.0.wrapping_add(relative_coord.0);
        let adj_col = coord.1.wrapping_add(relative_coord.1);

        if adj_row < h && adj_col < w {
            Some((adj_row, adj_col))
        } else {
            None
        }
    };

    let mut queue = VecDeque::<((usize, usize), usize)>::from([(start, 0)]);
    while let Some((coord, cost)) = queue.pop_front() {
        if cost_mat[coord].is_some() {
            continue;
        }

        cost_mat[coord] = Some(cost);

        for relative_coord in DIFFS {
            let Some(adj_coord) = calc_adj_coord(coord, relative_coord) else {
                continue;
            };

            let mut add_cost = 0;

            if ss[adj_coord.0][adj_coord.1] == '#' {
                add_cost = 1;
            }

            if add_cost == 0 {
                queue.push_front((adj_coord, cost + add_cost));
            } else {
                queue.push_back((adj_coord, cost + add_cost));
            }

            let Some(adj_coord) = calc_adj_coord(adj_coord, relative_coord) else {
                continue;
            };

            if ss[adj_coord.0][adj_coord.1] == '#' {
                add_cost = 1;
            }

            if add_cost == 0 {
                queue.push_front((adj_coord, cost + add_cost));
            } else {
                queue.push_back((adj_coord, cost + add_cost));
            }
        }
    }

    println!("{}", cost_mat[goal].unwrap());
}
