use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| if solve() { "Yes" } else { "No" })
        .join("\n");
    println!("{}", output);
}

fn solve() -> bool {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![false; 1 << n];
    dp[0] = true;
    for bits in 0..1 << n {
        if !dp[bits] {
            continue;
        }

        for bit in 0..n {
            let mixed_bits = bits | (1 << bit);
            if s[mixed_bits - 1] == '0' {
                dp[mixed_bits] = true;
            }
        }
    }

    dp[(1 << n) - 1]
}
