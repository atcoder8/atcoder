use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        ab: [(usize, usize); n],
    }

    let ab: HashSet<(usize, usize)> = ab.iter().cloned().collect();

    let mut ans = 0;
    let mut dp = vec![vec![0_usize; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            if ab.contains(&(i, j)) {
                continue;
            }

            dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
            ans += dp[i][j];
        }
    }

    println!("{}", ans);
}
