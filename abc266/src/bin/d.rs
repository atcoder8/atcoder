fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let mut txa = Vec::new();
    for _ in 0..n {
        txa.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<i64>().unwrap(),
            )
        });
    }

    // dp[i][j]: iターン目に座標jにいる
    let mut dp = vec![vec![-2e18 as i64; 5]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        let prev_t = if i == 0 { 0 } else { txa[i - 1].0 };
        let (t, x, a) = txa[i];

        for j in 0..5 {
            for k in 0..5 {
                if j.max(k) - j.min(k) <= t - prev_t {
                    dp[i + 1][j] = dp[i + 1][j].max(dp[i][k]);
                }
            }
            if j == x {
                dp[i + 1][j] += a;
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
