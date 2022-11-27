use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
        tt: [Chars; h],
    }

    let mut ss_t = (0..w)
        .map(|j| (0..h).map(|i| ss[i][j]).collect_vec())
        .collect_vec();
    ss_t.sort_unstable();

    let mut tt_t = (0..w)
        .map(|j| (0..h).map(|i| tt[i][j]).collect_vec())
        .collect_vec();
    tt_t.sort_unstable();

    println!("{}", if ss_t == tt_t { "Yes" } else { "No" });
}
