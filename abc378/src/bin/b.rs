use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        qr: [(usize, usize); n],
        q: usize,
        td: [(Usize1, usize); q],
    }

    let solve = |t: usize, d: usize| {
        let (q, r) = qr[t];
        let mut date = d / q * q + r;
        if date >= d + q {
            date -= q;
        } else if date < d {
            date += q;
        }

        date
    };

    let ans = td.iter().map(|&(t, d)| solve(t, d)).join("\n");
    println!("{}", ans);
}
