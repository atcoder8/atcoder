use std::{cmp::Reverse, collections::BTreeSet};

use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve()).join("\n"));
}

fn solve() -> usize {
    input! {
        (n, m): (usize, usize),
        mut aa: [usize; n],
        bb: [usize; n],
    }

    let mut sum = aa.iter().sum::<usize>() + bb.iter().sum::<usize>();

    aa.sort_unstable_by_key(|&a| Reverse(a));

    let mut bb = BTreeSet::from_iter(enumerate(bb).map(|(i, b)| (b, i)));
    for &a in &aa {
        match bb.range((m - a, 0)..).next() {
            Some(&(b, i)) => {
                sum -= m;
                bb.remove(&(b, i));
            }
            None => break,
        }
    }

    sum
}
