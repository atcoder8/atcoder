use std::collections::BTreeSet;

use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        pp: [Usize1; n],
    }

    let mut p_to_i = vec![0; n];
    for (i, &p) in enumerate(&pp) {
        p_to_i[p] = i;
    }

    let mut set = BTreeSet::new();
    for &i in &p_to_i[..k] {
        set.insert(i);
    }

    let mut ans = set.last().unwrap() - set.first().unwrap();

    for p in 0..n - k {
        set.remove(&p_to_i[p]);
        set.insert(p_to_i[p + k]);

        ans = ans.min(set.last().unwrap() - set.first().unwrap());
    }

    println!("{}", ans);
}
