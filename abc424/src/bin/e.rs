// unfinished

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> f64 {
    input! {
        (n, k, x): (usize, u64, u64),
        mut aa: [f64; n],
    }

    aa.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());

    let is_ok = |bound: f64| {
        let mut sum = 0_u64;
        let mut min_level = 0_u32;
        for &a in &aa {
            let divided = (a / bound).floor() as u64;
            let mut level = min_level;
            while 2_u64.pow(level + 1) <= divided {
                level += 1;
            }
            min_level = level;
            sum += 2_u64.pow(level) - 1
        }

        sum >= x && 2 * sum - x >= k
    };

    let mut ok = 0.0_f64;
    let mut ng = 10_f64.powi(9) + 1.0;
    while (ok - ng).abs() > 1e-11 {
        let mid = (ok + ng) / 2.0;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ok
}
