use std::collections::VecDeque;

use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (h, w): (usize, usize),
        ccc: [Chars; h],
    }

    let (start_r, start_c) = (|| {
        for i in 0..h {
            for j in 0..w {
                if ccc[i][j] == 'S' {
                    return Some((i, j));
                }
            }
        }

        None
    })()
    .unwrap();

    let mut que = VecDeque::new();
    let mut color = vec![vec![0; w]; h];

    for (i, &(diff_r, diff_c)) in DIFFS.iter().enumerate() {
        let (next_r, next_c) = (start_r.wrapping_add(diff_r), start_c.wrapping_add(diff_c));

        if next_r < h && next_c < w {
            que.push_back((next_r, next_c));
            color[next_r][next_c] = i + 1;
        }
    }

    while let Some(cur_coord) = que.pop_front() {
        let (cur_r, cur_c) = cur_coord;
        let cur_color = color[cur_r][cur_c];

        for &(diff_r, diff_c) in &DIFFS {
            let (next_r, next_c) = (cur_r.wrapping_add(diff_r), cur_c.wrapping_add(diff_c));

            if next_r >= h || next_c >= w || ccc[next_r][next_c] != '.' {
                continue;
            }

            let next_color = &mut color[next_r][next_c];

            if *next_color == 0 {
                *next_color = cur_color;
                que.push_back((next_r, next_c));
            } else if *next_color != cur_color {
                return true;
            }
        }
    }

    false
}
