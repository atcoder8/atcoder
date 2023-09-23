use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        k: Usize1,
    }

    let ans = (2..(1 << 10))
        .map(|bit| {
            (0..=9)
                .filter(|&i| (bit >> i) & 1 == 1)
                .rev()
                .fold(0_usize, |acc, x| 10 * acc + x)
        })
        .sorted()
        .nth(k)
        .unwrap();
    println!("{}", ans);
}
