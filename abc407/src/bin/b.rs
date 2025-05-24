use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (x, y): (u8, u8),
    }

    let cnt = iproduct!(1..=6, 1..=6)
        .filter(|&(a, b)| a + b >= x || a.abs_diff(b) >= y)
        .count();
    let ans = cnt as f64 / 36.0;
    println!("{}", ans);
}
