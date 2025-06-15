use itertools::Itertools;
use num::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        match solve() {
            Some(xx) => println!("Yes\n{}", xx.iter().join(" ")),
            None => println!("No"),
        }
    }
}

fn solve() -> Option<Vec<i64>> {
    input! {
        n: usize,
        mut aa: [i64; n],
        mut bb: [i64; n],
    }

    let gcd_a = aa.iter().cloned().reduce(|a1, a2| a1.gcd(&a2)).unwrap();
    aa.iter_mut().for_each(|a| *a /= gcd_a);

    let gcd_b = bb.iter().cloned().reduce(|b1, b2| b1.gcd(&b2)).unwrap();
    bb.iter_mut().for_each(|b| *b /= gcd_b);

    let pos1 = (0..n).find(|&i| aa[i] != bb[i])?;
    let (a1, b1) = (aa[pos1], bb[pos1]);
    let pos2 = (0..n).find(|&i| aa[i] * b1 != a1 * bb[i]).unwrap();
    let (a2, b2) = (aa[pos2], bb[pos2]);

    let mut xx = vec![0; n];
    xx[pos1] = a2 + b2;
    xx[pos2] = -(a1 + b1);

    if a2 * b1 > a1 * b2 {
        xx[pos1] = -xx[pos1];
        xx[pos2] = -xx[pos2];
    }

    Some(xx)
}
