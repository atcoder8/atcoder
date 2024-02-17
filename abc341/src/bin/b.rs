use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
        st: [(usize, usize); n - 1],
    }

    for (i, &(s, t)) in enumerate(&st) {
        aa[i + 1] += aa[i] / s * t;
    }

    println!("{}", aa[n - 1]);
}
