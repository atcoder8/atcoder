use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let mut hor_counts = vec![0; n];
    let mut ver_counts = vec![0; n];
    for (i, j) in iproduct!(0..n, 0..n) {
        if ss[i][j] == 'o' {
            hor_counts[i] += 1;
            ver_counts[j] += 1;
        }
    }

    let mut ans = 0_usize;
    for (i, j) in iproduct!(0..n, 0..n) {
        if ss[i][j] == 'o' {
            ans += (hor_counts[i] - 1) * (ver_counts[j] - 1);
        }
    }

    println!("{}", ans);
}
