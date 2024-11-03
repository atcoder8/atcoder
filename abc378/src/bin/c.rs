use std::collections::BTreeMap;

use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut bb = enumerate(&aa).scan(BTreeMap::new(), |map, (i, &a)| {
        Some(map.insert(a, i + 1).map_or(-1, |b| b as i32))
    });
    println!("{}", bb.join(" "));
}
