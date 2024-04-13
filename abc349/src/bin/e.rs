use std::iter::zip;

use proconio::input;

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn main() {
    input! {
        aaa: [i64; 9],
    }

    let ans = rec(&aaa, &[None; 9], 0);
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}

fn judge(grid: &[i64], colors: &[Option<bool>]) -> Option<bool> {
    for line in LINES {
        if line.iter().all(|&idx| colors[idx] == Some(true)) {
            return Some(true);
        }

        if line.iter().all(|&idx| colors[idx] == Some(false)) {
            return Some(false);
        }
    }

    if colors.iter().any(|color| color.is_none()) {
        return None;
    }

    let mut score1 = 0;
    let mut score2 = 0;
    for (&val, &color) in zip(grid, colors) {
        if color.unwrap() {
            score1 += val;
        } else {
            score2 += val;
        }
    }

    Some(score1 > score2)
}

fn rec(grid: &[i64], colors: &[Option<bool>], turn: usize) -> bool {
    if let Some(win) = judge(grid, colors) {
        return win;
    }

    let parity = turn % 2 == 0;

    let any = (0..9).filter(|&i| colors[i].is_none()).any(|i| {
        let mut next_colors = colors.to_owned();
        next_colors[i] = Some(parity);

        rec(grid, &next_colors, turn + 1) == parity
    });

    if any {
        parity
    } else {
        !parity
    }
}
