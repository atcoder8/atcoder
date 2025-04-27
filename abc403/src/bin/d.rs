use proconio::input;

const MAX: usize = 10_usize.pow(6);

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, d): (usize, usize),
        aa: [usize; n],
    }

    let mut counts = vec![0_usize; MAX + 1];
    for &a in &aa {
        counts[a] += 1;
    }

    if d == 0 {
        return counts.iter().map(|&cnt| cnt.saturating_sub(1)).sum();
    }

    let mut sum = 0_usize;
    for start in 0..d {
        let mut dp = [0_usize; 2];
        for &cnt in counts[start..].iter().step_by(d) {
            dp = [dp[0].min(dp[1]) + cnt, dp[0]];
        }

        sum += dp[0].min(dp[1]);
    }

    sum
}
