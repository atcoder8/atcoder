use std::collections::BTreeSet;

use itertools::enumerate;
use proconio::input;

fn main() {
    let ans = match solve() {
        Some(ans) => ans.to_string(),
        None => "-1".to_string(),
    };
    println!("{}", ans);
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        mut bb: [usize; m],
    }

    let mut set: BTreeSet<(usize, usize)> = enumerate(&aa).map(|(i, &a)| (a, i)).collect();

    let mut cost = 0;
    for &b in &bb {
        let elem = *set.range((b, 0)..).next()?;
        set.remove(&elem);
        cost += elem.0;
    }

    Some(cost)
}
