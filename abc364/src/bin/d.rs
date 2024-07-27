use itertools::Itertools;
use proconio::input;
use superslice::Ext;

const INF: i64 = 10_i64.pow(10);

fn main() {
    input! {
        (n, q): (usize, usize),
        mut aa: [i64; n],
        bk: [(i64, usize); q],
    }

    aa.sort_unstable();

    let mut rev = aa.clone();
    rev.reverse();

    let find_dist = |b: i64, k: usize| {
        let pivot = aa.lower_bound(&b);
        let left_aa = &rev[n - pivot..];
        let right_aa = &aa[pivot..];

        let mut ok = INF;
        let mut ng = -1;

        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;

            let within_num = left_aa.upper_bound_by_key(&mid, |x| (x - b).abs())
                + right_aa.upper_bound_by_key(&mid, |x| (x - b).abs());

            if within_num >= k {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    };

    let ans = bk.iter().map(|&(b, k)| find_dist(b, k)).join("\n");
    println!("{}", ans);
}
