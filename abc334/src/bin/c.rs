use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; k],
    }

    println!("{}", solve(n, aa));
}

fn solve(_n: usize, aa: Vec<usize>) -> usize {
    let k = aa.len();

    let mut dp = vec![0; k + 1];
    for (i, window) in enumerate(aa.windows(2)) {
        let mut next_cost = dp[i] + window[1] - window[0];
        if i % 2 == 1 && dp[i + 1] < next_cost {
            next_cost = dp[i + 1];
        }

        dp[i + 2] = next_cost;
    }

    dp[k]
}
