use std::{cmp::Reverse, iter::zip};

use itertools::{enumerate, rev, Itertools};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        let ans = solve();
        println!(
            "{}\n{}",
            ans.len(),
            ans.iter().map(|elem| elem + 1).join(" ")
        );
    }
}

fn calc_lis_by_tail<T>(seq: &[T]) -> Vec<usize>
where
    T: Copy + Ord,
{
    let mut lis_by_prefix = vec![0; seq.len()];
    let mut dp = vec![];
    for (i, &elem) in enumerate(seq) {
        let pos = dp.lower_bound(&elem);

        if pos < dp.len() {
            dp[pos] = elem;
        } else {
            dp.push(elem);
        }

        lis_by_prefix[i] = pos + 1;
    }

    lis_by_prefix
}

fn solve() -> Vec<usize> {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let forward = calc_lis_by_tail(&aa);
    let mut backward = calc_lis_by_tail(&rev(&aa).map(Reverse).collect_vec());
    backward.reverse();

    let lis = *forward.iter().max().unwrap();

    zip(forward, backward)
        .positions(|(len1, len2)| len1 + len2 - 1 == lis)
        .collect()
}
