use itertools::join;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        bb: [usize; m],
    }

    let mut aa_idx = vec![n + 1; n];
    let mut bb_idx = vec![m + 1; m];

    let mut whole_idx = 0;
    let mut a_idx = 0;
    let mut b_idx = 0;

    while a_idx < n && b_idx < m {
        if aa[a_idx] < bb[b_idx] {
            aa_idx[a_idx] = whole_idx + 1;
            a_idx += 1;
        } else {
            bb_idx[b_idx] = whole_idx + 1;
            b_idx += 1;
        }
        whole_idx += 1;
    }

    for i in a_idx..n {
        aa_idx[i] = whole_idx + 1;
        whole_idx += 1;
    }
    for i in b_idx..m {
        bb_idx[i] = whole_idx + 1;
        whole_idx += 1;
    }

    println!("{}", join(aa_idx, " "));
    println!("{}", join(bb_idx, " "));
}
