use itertools::{enumerate, Itertools};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        xx: [i64; n],
        pp: [usize; n],
        q: usize,
        lr: [(i64, i64); q],
    }

    let mut acc = vec![0_usize; n + 1];
    for (i, &p) in enumerate(&pp) {
        acc[i + 1] = acc[i] + p;
    }

    let solve = |l: i64, r: i64| {
        let lower_bound = xx.lower_bound(&l);
        let upper_bound = xx.upper_bound(&r);

        acc[upper_bound] - acc[lower_bound]
    };

    let ans = lr.iter().map(|&(l, r)| solve(l, r)).join("\n");
    println!("{}", ans);
}
