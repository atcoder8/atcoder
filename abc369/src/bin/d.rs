use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut dp = [0, aa[0]];
    for &a in &aa[1..] {
        dp = [dp[0].max(dp[1] + 2 * a), dp[1].max(dp[0] + a)];
    }

    println!("{}", dp[0].max(dp[1]));
}
