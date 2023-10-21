use itertools::Itertools;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 8] = [
    (!0, !0),
    (!0, 0),
    (!0, 1),
    (0, !0),
    (0, 1),
    (1, !0),
    (1, 0),
    (1, 1),
];

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let mut ans = 0;
    let mut visited = vec![vec![false; w]; h];
    for (i, j) in (0..h).cartesian_product(0..w) {
        if sss[i][j] == '.' || visited[i][j] {
            continue;
        }

        ans += 1;

        let mut stack = vec![(i, j)];
        while let Some((row, col)) = stack.pop() {
            if visited[row][col] {
                continue;
            }

            visited[row][col] = true;

            for (diff_row, diff_col) in DIFFS {
                let (next_row, next_col) = (row.wrapping_add(diff_row), col.wrapping_add(diff_col));

                if next_row >= h || next_col >= w {
                    continue;
                }

                if next_row < h && next_col < w && sss[next_row][next_col] == '#' {
                    stack.push((next_row, next_col));
                }
            }
        }
    }

    println!("{}", ans);
}
