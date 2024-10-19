use std::collections::BTreeSet;

use itertools::{enumerate, izip, Itertools};
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let ans = (0..t).map(|_| solve()).join("\n");
    println!("{}", ans);
}

fn solve() -> usize {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
        bb: [usize; n],
    }

    let sorted_bi = enumerate(&bb)
        .map(|(i, &b)| (b, i))
        .sorted_unstable_by_key(|v| v.0)
        .collect_vec();

    let mut sum_small_b = sorted_bi[..k].iter().map(|v| v.0).sum::<usize>();
    let mut large_b: BTreeSet<(usize, usize)> = sorted_bi[k..].iter().cloned().collect();

    let mut abi = enumerate(izip!(&aa, &bb))
        .map(|(i, (&a, &b))| (a, b, i))
        .sorted_unstable_by_key(|v| v.0)
        .collect_vec();

    // 使用できるAの範囲を削減していく
    let mut ans = abi.last().unwrap().0 * sum_small_b;
    while abi.len() > k {
        let (_, b, i) = abi.pop().unwrap();
        if large_b.contains(&(b, i)) {
            large_b.remove(&(b, i));
        } else {
            sum_small_b -= b;
            let first_large_b = large_b.pop_first().unwrap().0;
            sum_small_b += first_large_b;
        }

        ans = ans.min(abi.last().unwrap().0 * sum_small_b);
    }

    ans
}
