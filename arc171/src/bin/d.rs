use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (p, _b, mut n, m): (usize, usize, usize, usize),
        lr: [(Usize1, usize); m],
    }

    n += 1;

    let mut graph = vec![vec![false; n]; n];
    for &(l, r) in &lr {
        graph[l][r] = true;
        graph[r][l] = true;
    }

    let mut dp = vec![n; 1 << n];
    for bit in 1..1 << n {
        let nodes = (0..n).filter(|&i| bit >> i & 1 == 1).collect_vec();
        if nodes
            .iter()
            .tuple_combinations()
            .all(|(&u, &v)| !graph[u][v])
        {
            dp[bit] = 1;
            continue;
        }

        let mut sub_bit = (bit - 1) & bit;
        while sub_bit != 0 {
            dp[bit] = dp[bit].min(dp[sub_bit] + dp[bit ^ sub_bit]);
            sub_bit = sub_bit - 1 & bit;
        }
    }

    let ans = dp[(1 << n) - 1] <= p;
    println!("{}", if ans { "Yes" } else { "No" });
}
