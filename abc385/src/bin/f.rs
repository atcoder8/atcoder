// unfinished

use itertools::Itertools;
use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<f64> {
    input! {
        n: usize,
        xh: [(usize, usize); n],
    }

    if xh
        .iter()
        .tuple_windows()
        .all(|(&(x1, y1), &(x2, y2))| y1 * x2 < x1 * y2)
    {
        return None;
    }

    let mut ok = 2e18;
    let mut ng = 0.0_f64;
    while (ok - ng).abs() > 1e-10 {
        let mid = (ok + ng) / 2.0;

        if xh
            .iter()
            .map(|&(x, h)| (x as f64, h as f64))
            .tuple_windows()
            .all(|((x1, y1), (x2, y2))| (y1 - mid) * x2 < x1 * (y2 - mid))
        {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    Some(ng)
}
