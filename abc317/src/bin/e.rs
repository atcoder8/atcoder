use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [Chars; h],
    }

    let is_empty_square = |i: usize, j: usize| ['.', 'S', 'G'].contains(&aaa[i][j]);

    let mut passable = vec![vec![true; w]; h];

    for i in 0..h {
        for j in 0..w {
            if !is_empty_square(i, j) {
                passable[i][j] = false;
            }
        }
    }

    for i in 0..h {
        let mut seen = false;
        for j in 0..w {
            if !is_empty_square(i, j) {
                seen = false;
            }

            if aaa[i][j] == '>' {
                seen = true;
            }

            if seen {
                passable[i][j] = false;
            }
        }
    }

    for i in 0..h {
        let mut seen = false;
        for j in (0..w).rev() {
            if !is_empty_square(i, j) {
                seen = false;
            }

            if aaa[i][j] == '<' {
                seen = true;
            }

            if seen {
                passable[i][j] = false;
            }
        }
    }

    for j in 0..w {
        let mut seen = false;
        for i in 0..h {
            if !is_empty_square(i, j) {
                seen = false;
            }

            if aaa[i][j] == 'v' {
                seen = true;
            }

            if seen {
                passable[i][j] = false;
            }
        }
    }

    for j in 0..w {
        let mut seen = false;
        for i in (0..h).rev() {
            if !is_empty_square(i, j) {
                seen = false;
            }

            if aaa[i][j] == '^' {
                seen = true;
            }

            if seen {
                passable[i][j] = false;
            }
        }
    }

    let find_coord = |c: char| {
        for (i, j) in (0..h).cartesian_product(0..w) {
            if aaa[i][j] == c {
                return (i, j);
            }
        }

        panic!()
    };

    let start_coord = find_coord('S');
    let goal_coord = find_coord('G');

    let mut queue = VecDeque::from(vec![start_coord]);
    let mut dists = vec![vec![h * w; w]; h];
    dists[start_coord.0][start_coord.1] = 0;
    while let Some((row, col)) = queue.pop_front() {
        let candidate_dist = dists[row][col] + 1;

        for (diff_row, diff_col) in DIFFS {
            let next_row = row.wrapping_add(diff_row);
            let next_col = col.wrapping_add(diff_col);

            if next_row >= h || next_col >= w || !passable[next_row][next_col] {
                continue;
            }

            if candidate_dist < dists[next_row][next_col] {
                dists[next_row][next_col] = candidate_dist;
                queue.push_back((next_row, next_col));
            }
        }
    }

    let ans = dists[goal_coord.0][goal_coord.1];
    if ans < h * w {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
