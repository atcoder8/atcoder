use std::collections::VecDeque;

use ndarray::Array2;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        n: usize,
        grid: [Chars; n],
    }

    let calc_dist =
        |target_color: char, start_coord: (usize, usize), goal_coord: (usize, usize)| {
            let mut dist_mat = Array2::from_elem((n, n), n.pow(2));
            let mut queue = VecDeque::from([(start_coord, 0)]);

            while let Some((coord, cand_dist)) = queue.pop_front() {
                if dist_mat[coord] <= cand_dist {
                    continue;
                }

                dist_mat[coord] = cand_dist;

                let (row, col) = coord;

                for (diff_row, diff_col) in DIFFS {
                    let next_row = row.wrapping_add(diff_row);
                    let next_col = col.wrapping_add(diff_col);

                    if next_row < n && next_col < n {
                        if grid[next_row][next_col] == target_color {
                            queue.push_front(((next_row, next_col), cand_dist));
                        } else {
                            queue.push_back(((next_row, next_col), cand_dist + 1));
                        }
                    }
                }
            }

            dist_mat[goal_coord]
        };

    let ans = calc_dist('R', (0, 0), (n - 1, n - 1)) + calc_dist('B', (0, n - 1), (n - 1, 0));
    println!("{}", ans);
}
