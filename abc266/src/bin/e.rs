fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    // dp[i][j]: i回ダイスを振って最後にjが出た時の期待値
    let mut dp = vec![vec![0.0; 6]; n];

    for i in 0..6 {
        dp[n - 1][i] = (i + 1) as f64;
    }

    for i in (0..(n - 1)).rev() {
        for j in 0..6 {
            let p = dp[i + 1].iter().sum::<f64>() / 6.0;
            dp[i][j] = ((j + 1) as f64).max(p);
        }
    }

    let mut ans = 0.0;
    for vec in dp.iter() {
        let v = vec.iter().sum::<f64>() / 6.0;
        if v > ans {
            ans = v;
        }
    }

    println!("{}", ans);
}
