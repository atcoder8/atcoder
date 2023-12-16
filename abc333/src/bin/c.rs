use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut repunits = vec![0_usize];
    for len in 1..=15 {
        repunits.push(10 * repunits[len - 1] + 1);
    }

    let ans = iproduct!(1..=15, 1..=15, 1..=15)
        .map(|(i, j, k)| repunits[i] + repunits[j] + repunits[k])
        .sorted_unstable()
        .dedup()
        .nth(n - 1)
        .unwrap();
    println!("{}", ans);
}
