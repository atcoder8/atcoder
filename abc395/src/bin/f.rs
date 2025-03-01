use proconio::input;

const MAX: usize = 10_usize.pow(9);

fn main() {
    input! {
        (n, x): (usize, usize),
        ud: [(usize, usize); n],
    }

    let is_ok = |h: usize| -> bool {
        let (u0, d0) = ud[0];

        if u0 + d0 < h {
            return false;
        }

        let rem = u0 + d0 - h;

        let mut prev_min_u = u0.saturating_sub(rem);
        let mut prev_max_u = u0 - rem.saturating_sub(d0);

        for &(u, d) in &ud[1..] {
            if u + d < h {
                return false;
            }

            let rem = u + d - h;

            let min_u = (u.saturating_sub(rem)).max(prev_min_u.saturating_sub(x));
            let max_u = (u - rem.saturating_sub(d)).min(prev_max_u + x);

            if min_u > max_u {
                return false;
            }

            prev_min_u = min_u;
            prev_max_u = max_u;
        }

        true
    };

    let mut ok = 0_usize;
    let mut ng = 2 * MAX + 1;

    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let sum_all_length = ud.iter().map(|&(u, d)| u + d).sum::<usize>();
    let ans = sum_all_length - ok * n;
    println!("{}", ans);
}
