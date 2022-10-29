use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        hh: [usize; n],
    }

    let ans = hh.iter().position_max().unwrap() + 1;
    println!("{}", ans);
}
