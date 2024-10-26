use std::collections::BTreeSet;

use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut lr: [(usize, usize); n],
    }

    lr.sort_unstable_by_key(|&(l, _)| l);

    let mut right_set: BTreeSet<(usize, usize)> =
        enumerate(&lr).map(|(i, &(_, r))| (r, i)).collect();

    let mut ans = 0_usize;
    let mut progress = 0_usize;
    for l in 1..=m {
        while progress < n && lr[progress].0 < l {
            right_set.remove(&(lr[progress].1, progress));
            progress += 1;
        }

        let min_r = right_set.first().map_or(m + 1, |&(r, _)| r);
        ans += min_r - l;
    }
    println!("{}", ans);
}
