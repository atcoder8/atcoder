use itertools::Itertools;
use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let mut seq = (0..n).collect_vec();
    while seq.next_permutation() {
        let ret = (0..(n - 1)).all(|i| {
            (0..m)
                .filter(|&j| ss[seq[i]][j] != ss[seq[i + 1]][j])
                .count()
                == 1
        });
        if ret {
            return true;
        }
    }

    false
}
