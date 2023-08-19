use proconio::input;

fn main() {
    const PENALTY_LIMIT: usize = 30;

    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    let mut dist_mat = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            dist_mat[i][j] = calc_dist(xy[i], xy[j]);
        }
    }

    // dp[i][j]: j個のチェックポイントを省略して地点iまで通過したときの最小距離
    let mut dp = vec![vec![1e9; PENALTY_LIMIT + 1]; n];
    dp[0][0] = 0.0;
    for i in 1..n {
        for j in 0..=PENALTY_LIMIT {
            for k in 0..=j.min(i - 1) {
                let dist = dp[i - 1 - k][j - k] + dist_mat[i - 1 - k][i];
                if dist < dp[i][j] {
                    dp[i][j] = dist;
                }
            }
        }
    }

    let mut ans = dp[n - 1][0];
    for j in 1..=PENALTY_LIMIT {
        let cost = dp[n - 1][j] + 2.0_f64.powi(j as i32 - 1);
        if cost < ans {
            ans = cost;
        }
    }
    println!("{}", ans);
}

pub fn calc_dist(coord1: (f64, f64), coord2: (f64, f64)) -> f64 {
    let (x1, y1) = coord1;
    let (x2, y2) = coord2;

    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}
