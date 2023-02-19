use num::Integer;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        (n, d, k): (usize, usize, usize),
    }

    let period = n / n.gcd(&d);
    let cycle_num = (k - 1) / period;
    let rem = (k - 1) % period;

    let ans = (d * rem + cycle_num) % n;
    println!("{}", ans);
}
