use itertools::join;
use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    if let Some(xx) = solve() {
        println!("Yes");
        println!("{}", join(xx, " "));
    } else {
        println!("No");
    }
}

fn solve() -> Option<Vec<i64>> {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut bb: Vec<i64> = aa.iter().rev().cumsum().collect();
    bb.reverse();
    let sum_bb: i64 = bb.iter().sum();

    if bb[0] != 0 {
        let mut xx = vec![if bb[0] < 0 { 1 } else { -1 } * (sum_bb - bb[0])];
        let abs_b0 = bb[0].abs();

        for i in 1..n {
            xx.push(xx[i - 1] + abs_b0);
        }

        return Some(xx);
    }

    if bb.iter().all(|&b| b <= 0) || bb.iter().all(|&b| b >= 0) {
        return None;
    }

    let mut yy = vec![1; n];

    if sum_bb >= 0 {
        let p = bb.iter().position(|&b| b == -1).unwrap();
        yy[p] = 1 + sum_bb;
    } else {
        let q = bb.iter().position(|&b| b == 1).unwrap();
        yy[q] = 1 - sum_bb;
    }

    Some(yy.iter().cumsum().collect())
}
