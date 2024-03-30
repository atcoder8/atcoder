use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let ans = aa
        .into_iter()
        .filter(|&a| a % k == 0)
        .map(|a| a / k)
        .join(" ");
    println!("{}", ans);
}
