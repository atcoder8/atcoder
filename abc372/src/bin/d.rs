use std::{cmp::Reverse, collections::BTreeSet};

use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        hh: [usize; n],
    }

    let mut imos = vec![0_i64; n];
    let mut indexes = BTreeSet::<usize>::new();
    for (j, _) in enumerate(&hh).sorted_unstable_by_key(|(_, &h)| Reverse(h)) {
        let left = indexes.range(..j).next_back().cloned().unwrap_or(0);
        imos[left] += 1;
        imos[j] -= 1;
        indexes.insert(j);
    }

    for i in 0..n - 1 {
        imos[i + 1] += imos[i];
    }

    println!("{}", imos.iter().join(" "));
}
