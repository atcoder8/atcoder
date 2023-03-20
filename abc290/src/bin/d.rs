use num::Integer;
use proconio::input;

fn main() {
    input! {
        t: usize,
        ndk: [(usize, usize, usize); t],
    }

    for &(n, d, k) in &ndk {
        let cycle_len = n / n.gcd(&d);
        let shift_num = (k - 1) / cycle_len;

        let ans = ((k - 1) * d + shift_num) % n;
        println!("{}", ans);
    }
}
