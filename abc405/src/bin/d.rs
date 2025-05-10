use std::collections::VecDeque;

use itertools::{izip, Itertools};
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

const DIFFS: [[usize; 2]; 4] = [[!0, 0], [0, !0], [0, 1], [1, 0]];

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let grid = Array2::from_shape_fn((h, w), |(row, col)| sss[row][col]);

    let start_coords: Vec<[usize; 2]> = grid
        .indexed_iter()
        .filter(|(_, &square_type)| square_type == 'E')
        .map(|(coord, _)| [coord.0, coord.1])
        .collect();

    let mut arrow_array = Array2::from_elem((h, w), '#');

    let mut queue: VecDeque<_> = start_coords.iter().map(|&coord| (coord, 'E')).collect();
    while let Some((coord, arrow)) = queue.pop_front() {
        if arrow_array[coord] != '#' {
            continue;
        }

        arrow_array[coord] = arrow;

        for (diffs, adj_arrow) in izip!(DIFFS, ['v', '>', '<', '^']) {
            let adj_coord: [usize; 2] = std::array::from_fn(|i| coord[i].wrapping_add(diffs[i]));
            if adj_coord[0] < h && adj_coord[1] < w && grid[adj_coord] != '#' {
                queue.push_back((adj_coord, adj_arrow));
            }
        }
    }

    let ans = arrow_array
        .axis_iter(Axis(0))
        .map(|line| line.iter().collect::<String>())
        .join("\n");
    println!("{}", ans);
}
