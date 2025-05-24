use std::cmp::min_by_key;

use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = min_by_key(a / b, (a + b - 1) / b, |k| a.abs_diff(b * k));
    println!("{}", ans);
}
