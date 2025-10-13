use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        nk: [(usize, usize); t],
    }

    for &(n, k) in &nk {
        let aa = solve(n, k);
        let output = aa.iter().join(" ");
        println!("{output}");
    }
}

fn solve(n: usize, k: usize) -> Vec<usize> {
    let build = |last: usize| {
        let mut rem = k;
        let mut aa = vec![last];
        for _ in 0..n - 1 {
            let last = *aa.last().unwrap();
            let curr = (last.saturating_sub(rem)).max(last / 2 + 1);
            aa.push(curr);
            rem -= last % curr;
        }
        aa.reverse();

        if rem == 0 {
            Some(aa)
        } else {
            None
        }
    };

    let max = 2 * k + 1;
    let mut ok = max;
    let mut ng = 0_usize;

    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if build(mid).is_some() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    build(ok).unwrap()
}
