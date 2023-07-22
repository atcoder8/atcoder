use im_rc::HashSet;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        ab: [(Usize1, Usize1); n],
    }

    let ab_set: HashSet<(usize, usize)> = ab.iter().cloned().collect();

    let mut dp = vec![vec![0_usize; w]; h];
    for i in 0..h {
        dp[i][0] = !ab_set.contains(&(i, 0)) as usize;
    }
    for j in 0..w {
        dp[0][j] = !ab_set.contains(&(0, j)) as usize;
    }

    for i in 0..h {
        for j in 0..w {
            if ab_set.contains(&(i, j)) {
                continue;
            }

            if i == 0 || j == 0 {
                dp[i][j] = 1;
                continue;
            }

            dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans += dp[i][j];
        }
    }
    println!("{}", ans);
}
