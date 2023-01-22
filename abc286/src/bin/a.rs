use itertools::join;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, p, q, r, _s): (usize, Usize1, usize, Usize1, usize),
        aa: [usize; n],
    }

    let mut bb = aa.clone();
    let size = q - p;

    for i in 0..size {
        bb.swap(p + i, r + i);
    }

    println!("{}", join(bb, " "));
}
