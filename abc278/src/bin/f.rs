// unfinished

use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "First" } else { "Second" });
}

fn solve() -> bool {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let mut dp = vec![vec![false; n]; 1 << n];

    for bit in 0..(1 << n) {
        for from in 0..n {
            if bit != 0 && !dp[bit][from] {
                continue;
            }

            for to in 0..n {
                if (bit >> to) & 1 == 1 {
                    continue;
                }

                if bit == 0 || *ss[from].last().unwrap() == *ss[to].first().unwrap() {
                    dp[bit | (1 << to)][to] = true;
                }
            }
        }
    }

    let cloned_dp = dp.clone();

    for bit in (1_usize..(1 << n)).rev() {
        let ones = bit.count_ones() as usize;

        if ones % 2 == 0 {
            for from in 0..n {
                dp[bit][from] &= (0..n)
                    .filter(|&to| cloned_dp[bit | (1 << to)][to])
                    .any(|to| dp[bit | (1 << to)][to]);
            }
        } else {
            for from in 0..n {
                dp[bit][from] &= (0..n)
                    .filter(|&to| cloned_dp[bit | (1 << to)][to])
                    .all(|to| dp[bit | (1 << to)][to]);
            }
        }
    }

    (0..n)
        .filter(|&to| cloned_dp[1 << to][to])
        .any(|to| dp[1 << to][to])
}
