use proconio::input;

fn main() {
    input! {
        n: usize,
        pp: [f64; n],
    }

    // dp[i]: コンテストをi個選択したときの第1項の分子の最大値
    let mut dp = vec![0.0];
    dp.reserve(n);

    for &p in &pp {
        dp.push(0.0);
        for k in (1..dp.len() - 1).rev() {
            let cand_numer = dp[k] * 0.9 + p;
            if cand_numer > dp[k + 1] {
                dp[k + 1] = cand_numer;
            }
        }

        dp[1] = dp[1].max(p);
    }

    let mut ans = -2e18_f64;
    let mut denom = 0.0;
    for k in 1..=n {
        denom = denom * 0.9 + 1.0;

        let rate = dp[k] / denom - 1200.0 / (k as f64).sqrt();
        ans = ans.max(rate);
    }

    println!("{}", ans);
}
