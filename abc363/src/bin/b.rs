use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (n, t, p): (usize, usize, usize),
        mut ll: [usize; n],
    }

    ll.sort_unstable_by_key(|&l| Reverse(l));
    let ans = t.saturating_sub(ll[p - 1]);
    println!("{}", ans);
}
