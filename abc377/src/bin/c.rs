use itertools::{iproduct, Itertools};
use proconio::input;

const DIFF: [i64; 4] = [-2, -1, 1, 2];

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(i64, i64); m],
    }

    let in_range = |coord: (i64, i64)| {
        let (row, col) = coord;
        1 <= row && row <= n as i64 && 1 <= col && col <= n as i64
    };

    let diffs = iproduct!(DIFF, DIFF)
        .filter(|&(dx, dy)| dx.abs() != dy.abs())
        .collect_vec();

    let mut filled = ab.clone();
    for &(a, b) in &ab {
        for &(dx, dy) in &diffs {
            let coord = (a + dx, b + dy);
            if in_range(coord) {
                filled.push(coord);
            }
        }
    }
    filled.sort_unstable();
    filled.dedup();

    let ans = n.pow(2) - filled.len();
    println!("{}", ans);
}
