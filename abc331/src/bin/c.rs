use itertools::{enumerate, Itertools};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut bb = aa.clone();
    bb.sort_unstable();

    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + bb[i];
    }

    let mut ans = vec![0; n];
    for (i, &a) in enumerate(&aa) {
        let pos = bb.upper_bound(&a);
        ans[i] = acc[n] - acc[pos];
    }

    println!("{}", ans.iter().join(" "));
}
