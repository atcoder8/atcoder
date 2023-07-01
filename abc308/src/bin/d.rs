use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

const SNUKE: [char; 5] = ['s', 'n', 'u', 'k', 'e'];

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let mut visited = vec![vec![false; w]; h];
    let ans = rec(&ss, 0, 0, 0, &mut visited);
    println!("{}", if ans { "Yes" } else { "No" });
}

fn rec(
    ss: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    pos: usize,
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    let h = ss.len();
    let w = ss[0].len();

    if ss[row][col] != SNUKE[pos] {
        return false;
    }

    if row == h - 1 && col == w - 1 {
        return true;
    }

    visited[row][col] = true;

    let mut ret = false;
    for &(diff_x, diff_y) in &DIFFS {
        let next_row = row.wrapping_add(diff_x);
        let next_col = col.wrapping_add(diff_y);
        let next_pos = (pos + 1) % SNUKE.len();

        if next_row < h && next_col < w && !visited[next_row][next_col] {
            ret |= rec(ss, next_row, next_col, next_pos, visited);
        }
    }

    ret
}
