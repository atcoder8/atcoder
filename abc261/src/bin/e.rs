fn main() {
    let (n, c) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let mut ta = Vec::new();
    for _ in 0..n {
        ta.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        });
    }

    let mut dp = vec![(0, (1 << 30) - 1); n + 1];

    let mut x = c;

    for (i, &(t, a)) in ta.iter().enumerate() {
        dp[i + 1] = if t == 1 {
            (dp[i].0 & a, dp[i].1 & a)
        } else if t == 2 {
            (dp[i].0 | a, dp[i].1 | a)
        } else {
            (dp[i].0 ^ a, dp[i].1 ^ a)
        };

        x = (0..30)
            .map(|j| {
                if (x >> j) & 1 == 0 {
                    dp[i + 1].0 & (1 << j)
                } else {
                    dp[i + 1].1 & (1 << j)
                }
            })
            .sum();
        
        println!("{}", x);
    }
}
