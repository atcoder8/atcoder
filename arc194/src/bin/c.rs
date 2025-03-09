// unfinished

use std::cmp::Reverse;

use itertools::izip;
use proconio::input;

// A[i]=1,B[i]=1を満たすiに対し、C[i]が大きい場合にA[i]を反転
// 1->0を優先
//

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
        cc: [usize; n],
    }

    let init_cost = izip!(&aa, &cc).map(|(a, c)| a * c).sum::<usize>();

    let mut dec_cc = vec![];
    let mut inc_cc = vec![];
    let mut one_cc = vec![];
    for (&a, &b, &c) in izip!(&aa, &bb, &cc) {
        match (a, b) {
            (1, 0) => dec_cc.push(c),
            (0, 1) => inc_cc.push(c),
            (1, 1) => one_cc.push(c),
            _ => {}
        }
    }
    dec_cc.sort_unstable_by_key(|&c| Reverse(c));
    inc_cc.sort_unstable();
    one_cc.sort_by_cached_key(|&c| Reverse(c));

    let mut sum_cost = 0_usize;

    let mut cost = init_cost;
    for &c in &dec_cc {
        cost -= c;
        sum_cost += cost;
    }
    for &c in &inc_cc {
        cost += c;
        sum_cost += cost;
    }

    println!("{}", sum_cost);
}
