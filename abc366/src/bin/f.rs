use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut ab: [(usize, usize); n],
    }

    ab.sort_unstable_by(|(a, b), (c, d)| ((a - 1) * d).cmp(&((c - 1) * b)));

    let mut dp = vec![0_usize; k + 1];
    dp[0] = 1;
    for &(a, b) in &ab {
        for i in (0..k).rev() {
            dp[i + 1] = dp[i + 1].max(dp[i] * a + b);
        }
    }

    println!("{}", dp[k]);
}
