use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        ab: [(usize, usize); n],
    }

    let mut dp = vec![false; x + 1];
    dp[0] = true;

    for i in 0..n {
        let (a, b) = ab[i];

        let mut next_dp = dp.clone();

        for cur_money in 0..=x {
            if !dp[cur_money] {
                continue;
            }

            for coin_cnt in 1..=b {
                let next_money = cur_money + a * coin_cnt;

                if next_money > x {
                    break;
                }

                next_dp[next_money] = true;
            }
        }

        dp = next_dp;
    }

    println!("{}", if dp[x] { "Yes" } else { "No" });
}
