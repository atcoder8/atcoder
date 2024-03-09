use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        mut aa: [usize],
        mut bb: [usize],
        mut cc: [usize],
        xx: [usize],
    }

    aa.sort_unstable();
    bb.sort_unstable();
    cc.sort_unstable();

    let combs = iproduct!(aa, bb, cc)
        .map(|(a, b, c)| a + b + c)
        .sorted_unstable()
        .collect_vec();

    for x in xx {
        let ans = combs.binary_search(&x).is_ok();
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
