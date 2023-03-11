use itertools::{join, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut called = vec![false; n];
    for i in 0..n {
        if !called[i] {
            called[aa[i]] = true;
        }
    }

    let ans = (0..n).filter(|&i| !called[i]).map(|i| i + 1).collect_vec();
    println!("{}\n{}", ans.len(), join(ans, " "));
}
