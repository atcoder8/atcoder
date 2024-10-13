use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let ans = (0..t)
        .map(|_| if solve() { "Yes" } else { "No" })
        .join("\n");
    println!("{}", ans);
}

fn solve() -> bool {
    input! {
        n: usize,
        mut aa: [i64; n],
    }

    let sum_a = aa.iter().sum::<i64>();
    let (q, r) = (sum_a / n as i64, sum_a % n as i64);

    for i in 0..n - 1 {
        let dest = q + (i >= n - r as usize) as i64;
        let diff = dest - aa[i];
        if diff < 0 {
            return false;
        }
        aa[i] += diff;
        aa[i + 1] -= diff;
    }

    aa.iter().tuple_windows().all(|(a1, a2)| a1 <= a2)
}
