// unfinished

use itertools::Itertools;
use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    let mut acc_gcd = 0;

    for &a in &aa {
        if aa[n - 1] > 2 * a {
            continue;
        }

        acc_gcd = acc_gcd.gcd(&a);
    }

    let bb = aa.iter().rev().map(|&a| aa[n - 1] - a).collect_vec();

    for b in &bb {
        if bb[n - 1] > 2 * b {
            continue;
        }

        acc_gcd = acc_gcd.gcd(&b);
    }

    let ans = if acc_gcd == 0 {
        aa[n - 1]
    } else {
        aa[0] + acc_gcd
    };

    println!("{}", aa[n - 1] - acc_gcd);
}
