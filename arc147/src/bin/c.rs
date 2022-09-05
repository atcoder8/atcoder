use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let (mut ll, mut rr): (Vec<usize>, Vec<usize>) = lr.into_iter().unzip();
    ll.sort_unstable_by_key(|&l| Reverse(l));
    rr.sort_unstable();

    let ans: usize = (0..(n / 2))
        .take_while(|&i| ll[i] > rr[i])
        .map(|i| (ll[i] - rr[i]) * (n - 1 - 2 * i))
        .sum();

    println!("{}", ans);
}
