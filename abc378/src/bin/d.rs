use itertools::iproduct;
use ndarray::prelude::*;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w, k): (usize, usize, usize),
        sss: [Chars; h],
    }

    let passable_grid = Array2::from_shape_fn((h, w), |(row, col)| sss[row][col] == '.');

    let count_routes = |start_coord: (usize, usize)| {
        if !passable_grid[start_coord] {
            return 0_usize;
        }

        let mut count = 0_usize;
        let mut coord_stack = vec![vec![start_coord]];
        while let Some(route) = coord_stack.pop() {
            if route.len() == k + 1 {
                count += 1;
                continue;
            }

            let (row, col) = *route.last().unwrap();
            for (diff_row, diff_col) in DIFFS {
                let adj_row = row.wrapping_add(diff_row);
                let adj_col = col.wrapping_add(diff_col);
                let adj_coord = (adj_row, adj_col);

                if adj_row < h
                    && adj_col < w
                    && passable_grid[adj_coord]
                    && !route.contains(&adj_coord)
                {
                    let mut next_route = route.clone();
                    next_route.push(adj_coord);
                    coord_stack.push(next_route);
                }
            }
        }

        count
    };

    let ans = iproduct!(0..h, 0..w)
        .map(|start_coord| count_routes(start_coord))
        .sum::<usize>();
    println!("{}", ans);
}
