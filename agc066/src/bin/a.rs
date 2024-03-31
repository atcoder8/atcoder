use itertools::{enumerate, iproduct, Itertools};
use ndarray::Array;
use proconio::input;

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (n, d): (usize, i64),
        aaa: [i64; n.pow(2)],
    }

    let mut grid = Array::from_shape_vec((n, n), aaa).unwrap();

    let sorted_coords = iproduct!(0..n, 0..n)
        .sorted_unstable_by_key(|&coord| grid[coord])
        .collect_vec();
    let mut label_mat = Array::from_elem((n, n), 0_usize);
    for (i, &coord) in enumerate(&sorted_coords) {
        label_mat[coord] = i;
    }

    for coord in sorted_coords {
        let mut stack = vec![coord];
        while let Some(coord) = stack.pop() {
            let label = label_mat[coord];
            let (row, col) = coord;

            for (diff_row, diff_col) in DIFFS {
                let next_row = row.wrapping_add(diff_row);
                let next_col = col.wrapping_add(diff_col);

                if next_row >= n || next_col >= n {
                    continue;
                }

                let next_coord = (next_row, next_col);

                let nei_label = label_mat[next_coord];

                if nei_label < label {
                    if grid[next_coord] > grid[coord] - d {
                        grid[next_coord] = grid[coord] + d;
                        stack.push(next_coord);
                    }
                } else {
                    if grid[next_coord] < grid[coord] + d {
                        grid[next_coord] = grid[coord] + d;
                        stack.push(next_coord);
                    }
                }
            }
        }
    }

    for row in 0..n {
        println!("{}", (0..n).map(|col| grid[(row, col)]).join(" "));
    }
}
