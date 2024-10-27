// unfinished

use itertools::{iproduct, Itertools};
use proconio::input;

// 固定されていない領域(2x2以上)を0個以上組み合わせる
// 4と6と9を組み合わせる

fn main() {
    input! {
        (n, q): (usize, usize),
        kk: [usize; q],
    }

    let mut flags = vec![false; n.pow(2) + 1];
    flags[0] = true;
    flags[n.pow(2)] = true;
    for (num_floating_rows, num_floating_cols) in iproduct!(2..=n, 2..=n) {
        flags[n.pow(2) - num_floating_rows * num_floating_cols] = true;
        if n - num_floating_rows >= 3 && n - num_floating_cols >= 3 {
            flags[n.pow(2) - num_floating_rows * num_floating_cols - 9] = true;
        }
    }

    let ans = kk
        .iter()
        .map(|&k| if flags[k] { "Yes" } else { "No" })
        .join("\n");
    println!("{}", ans);
}
