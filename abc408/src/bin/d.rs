use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve()).join("\n"));
}

fn solve() -> usize {
    input! {
        _n: usize,
        s: String,
    }

    let mut dp = [0_usize; 3];
    for c in s.chars() {
        dp = if c == '0' {
            [dp[0], dp[1].min(dp[0]) + 1, dp[2].min(dp[1])]
        } else {
            [dp[0] + 1, dp[1].min(dp[0]), dp[2].min(dp[1]) + 1]
        };
    }

    *dp.iter().min().unwrap()
}
