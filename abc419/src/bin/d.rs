use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (n, m): (usize, usize),
        mut s: Chars,
        mut t: Chars,
        lr: [(Usize1, usize); m],
    }

    let mut flip = vec![false; n + 1];
    for &(l, r) in &lr {
        flip[l] = !flip[l];
        flip[r] = !flip[r];
    }

    let mut parity = false;
    for i in 0..n {
        parity = parity != flip[i];
        if parity {
            std::mem::swap(&mut s[i], &mut t[i]);
        }
    }

    println!("{}", s.iter().join(""));
}
