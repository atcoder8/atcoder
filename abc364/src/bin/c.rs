use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (n, x, y): (usize, usize, usize),
        mut aa: [usize; n],
        mut bb: [usize; n],
    }

    aa.sort_unstable_by_key(|&a| Reverse(a));
    bb.sort_unstable_by_key(|&b| Reverse(b));

    let solve = |seq: &[usize], limit: usize| {
        let mut sum = 0_usize;
        let mut cnt = 0_usize;

        for &v in seq {
            if sum > limit {
                break;
            }

            sum += v;
            cnt += 1;
        }

        cnt
    };

    let ans = solve(&aa, x).min(solve(&bb, y));
    println!("{}", ans);
}
