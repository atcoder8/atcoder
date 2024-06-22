use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; 2 * n],
    }

    let is_ok = |color: usize| {
        let positions = aa.iter().positions(|&a| a == color).collect_vec();
        positions[1] - positions[0] == 2
    };

    let ans = (0..n).filter(|&i| is_ok(i)).count();
    println!("{}", ans);
}
