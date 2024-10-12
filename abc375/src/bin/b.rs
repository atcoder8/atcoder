use itertools::{chain, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let ans = chain!([&(0.0, 0.0)], &xy, [&(0.0, 0.0)])
        .tuple_windows()
        .map(|(&(x1, y1), &(x2, y2))| (x1 - x2).hypot(y1 - y2))
        .sum::<f64>();
    println!("{}", ans);
}
