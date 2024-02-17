use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
    }

    let is_ok = |x: usize| x / n + x / m - 2 * (x / n.lcm(&m)) >= k;

    let mut ok = 2 * 10_usize.pow(18);
    let mut ng = 0_usize;
    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
