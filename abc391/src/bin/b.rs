use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        sss: [Chars; n],
        ttt: [Chars; m],
    }

    let is_ok = |diff: (usize, usize)| {
        let (dr, dc) = diff;
        iproduct!(0..m, 0..m).all(|(row, col)| sss[row + dr][col + dc] == ttt[row][col])
    };

    let ans = iproduct!(0..=n - m, 0..=n - m)
        .find(|&diff| is_ok(diff))
        .unwrap();
    println!("{} {}", ans.0 + 1, ans.1 + 1);
}
