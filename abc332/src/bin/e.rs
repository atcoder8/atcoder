use std::f64::INFINITY;

use proconio::input;

fn main() {
    input! {
        (n, d): (usize, usize),
        ww: [f64; n],
    }

    let avg = ww.iter().sum::<f64>() / d as f64;

    let calc_squared_error = |bit: usize| {
        let bag_weight = (0..n)
            .filter(|&i| bit >> i & 1 == 1)
            .map(|i| ww[i])
            .sum::<f64>();

        (bag_weight - avg).powi(2)
    };

    // dp[k][S]: 集合Sに含まれるグッズをk+1個に分けるときの各福袋の重さと平均値との差の二乗の総和の最小値
    let mut dp = vec![vec![INFINITY; 1 << n]; d];
    for bit in 0..1 << n {
        dp[0][bit] = calc_squared_error(bit);
    }
    for k in 1..d {
        for bit in 0..1 << n {
            let mut sub_bit = bit;
            loop {
                let sum_squared_error = dp[0][sub_bit] + dp[k - 1][bit ^ sub_bit];
                dp[k][bit] = dp[k][bit].min(sum_squared_error);

                if sub_bit == 0 {
                    break;
                }

                sub_bit = sub_bit - 1 & bit;
            }
        }
    }

    let ans = dp[d - 1][(1 << n) - 1] / d as f64;
    println!("{}", ans);
}
