use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (a, b): (i64, i64),
    }

    let mut xx = vec![2 * a - b, 2 * b - a];
    if (a + b) % 2 == 0 {
        xx.push((a + b) / 2);
    }

    let ans = xx.iter().unique().count();
    println!("{}", ans);
}
