// unfinished

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut hwd: [(usize, usize, usize); n],
    }

    let rects = hwd
        .iter()
        .map(|&(h, w, d)| {
            let mut v = vec![h, w, d];
            v.sort_unstable();

            v
        })
        .sorted()
        .collect_vec();

    for i in 0..n {
        let (_, w1, d1) = hwd[i];
    }
}
