use std::collections::BTreeSet;

use itertools::chain;
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, s): (usize, usize),
        aa: [usize; n],
    }

    let sum = aa.iter().sum::<usize>();
    let rem = s % sum;

    let mut set = BTreeSet::from([0_usize]);
    let mut acc = 0_usize;
    for &a in chain!(&aa, &aa) {
        acc = (acc + a) % sum;
        if set.contains(&((acc + sum - rem) % sum)) {
            return true;
        }

        set.insert(acc);
    }

    false
}
