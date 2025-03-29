use std::collections::BTreeSet;

use amplify::confinement::Collection;
use itertools::{enumerate, izip, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    let ans = (0..t).map(|_| solve()).join("\n");
    println!("{}", ans);
}

fn solve() -> usize {
    input! {
        n: usize,
        aa: [Usize1; 2 * n],
    }

    let mut positions = vec![vec![]; 2 * n];
    for (i, &a) in enumerate(&aa) {
        positions[a].push(i);
    }

    let is_ok = |a: usize, b: usize| {
        [a, b]
            .into_iter()
            .all(|a| positions[a][1] - positions[a][0] != 1)
            && izip!(&positions[a], &positions[b]).all(|(&pos1, &pos2)| pos1.abs_diff(pos2) == 1)
    };

    let mut pairs = BTreeSet::new();

    let mut cnt = 0_usize;
    for (&a, &b) in aa.iter().tuple_windows() {
        if !pairs.contains(&(a, b)) && is_ok(a, b) {
            cnt += 1;
            pairs.push((a, b));
            pairs.push((b, a));
        }
    }
    cnt
}
