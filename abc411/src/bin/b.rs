use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        dd: [usize; n - 1],
    }

    let mut acc = vec![0_usize; n];
    for i in 0..n - 1 {
        acc[i + 1] = acc[i] + dd[i];
    }

    for i in 0..n - 1 {
        println!("{}", (i + 1..n).map(|j| acc[j] - acc[i]).join(" "));
    }
}
