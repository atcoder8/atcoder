use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, x): (usize, usize),
        vac: [(Usize1, usize, usize); n],
    }

    // ビタミンの種類ごとに分割してDP
    let mut dp = vec![vec![0_usize; x + 1]; 3];
    for &(v, a, c) in &vac {
        for next_cost in (c..=x).rev() {
            dp[v][next_cost] = dp[v][next_cost].max(dp[v][next_cost - c] + a);
        }
    }

    // 累積最大値に変換
    for v in 0..3 {
        for sum_c in 0..x {
            dp[v][sum_c + 1] = dp[v][sum_c + 1].max(dp[v][sum_c]);
        }
    }

    // ビタミン1とビタミン2の結果を合成
    let mut composited = vec![0_usize; x + 1];
    for cost1 in 0..=x {
        for cost2 in 0..=x - cost1 {
            let sum_cost = cost1 + cost2;
            composited[sum_cost] = composited[sum_cost].max(dp[0][cost1].min(dp[1][cost2]));
        }
    }

    // ビタミン1,2とビタミン3の結果を合成
    let mut ans = 0_usize;
    for cost1 in 0..=x {
        for cost2 in 0..=x - cost1 {
            ans = ans.max(composited[cost1].min(dp[2][cost2]));
        }
    }

    println!("{}", ans);
}
