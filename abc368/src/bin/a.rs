use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let ans = aa[n - k..].iter().chain(&aa[..n - k]).join(" ");
    println!("{}", ans);
}
