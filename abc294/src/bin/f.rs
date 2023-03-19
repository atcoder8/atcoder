use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        mut ab: [(f64, f64); n],
        mut cd: [(f64, f64); m],
    }

    let mut ok: f64 = 0.0;
    let mut ng: f64 = 1.0;

    while (ok - ng).abs() > 1e-12 {
        let mid = (ok + ng) / 2.0;

        let mut seq = cd.iter().map(|&(c, d)| c - (c + d) * mid).collect_vec();
        seq.sort_unstable_by(|x, y| x.partial_cmp(&y).unwrap());

        let small_num: usize = ab
            .iter()
            .map(|&(a, b)| seq.lower_bound_by(|&x| x.partial_cmp(&((a + b) * mid - a)).unwrap()))
            .sum();
        let large_num = n * m - small_num;

        if large_num >= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", 100.0 * ok);
}
