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
    let xx = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    };
    let mut cy = Vec::new();
    for _ in 0..m {
        cy.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<i64>().unwrap(),
            )
        });
    }

    let mut bonus = vec![0; n];
    for &(c, y) in cy.iter() {
        bonus[c - 1] = y;
    }

    let mut dp = vec![vec![-2e18 as i64; n + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..n {
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + xx[i] + bonus[j]);
            dp[i + 1][0] = dp[i + 1][0].max(dp[i][j]);
        }
    }

    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
