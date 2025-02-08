use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
        qq: [Usize1; n],
    }

    let mut q_to_i = vec![n; n];
    for (i, &q) in enumerate(&qq) {
        q_to_i[q] = i;
    }
    let ans = (0..n).map(|i| qq[pp[q_to_i[i]]] + 1).join(" ");
    println!("{}", ans);
}
