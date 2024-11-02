use std::collections::BTreeMap;

use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut bb = vec![None; n];
    let mut map = BTreeMap::new();
    for (i, &a) in enumerate(&aa) {
        if let Some(&pos) = map.get(&a) {
            bb[i] = Some(pos + 1);
        }

        map.insert(a, i);
    }

    let ans = bb
        .iter()
        .map(|&b| match b {
            Some(b) => b.to_string(),
            None => "-1".to_string(),
        })
        .join(" ");
    println!("{}", ans);
}
