use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, l, r): (usize, usize, usize),
        aa: [usize; n],
    }

    println!("{}", aa.iter().map(|&a| a.clamp(l, r)).join(" "));
}
