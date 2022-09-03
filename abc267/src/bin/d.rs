fn main() {
    let (n, m) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let aa = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    };

    // dp[i][j]: Aのi番目の要素まで使って長さjの数列を作ったときの最大値
    let mut dp = vec![vec![-2e18 as i64; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        dp[i + 1] = dp[i].clone();
        for j in 0..m {
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + (j + 1) as i64 * aa[i]);
        }
    }

    println!("{}", dp[n][m]);
}
