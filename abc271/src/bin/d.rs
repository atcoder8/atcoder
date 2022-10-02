use proconio::input;

fn main() {
    if let Some(ans) = solve() {
        println!("Yes\n{}", ans);
    } else {
        println!("No");
    }
}

fn solve() -> Option<String> {
    input! {
        (n, s): (usize, usize),
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![None; s + 1]; n + 1];
    dp[0][0] = Some('\0');

    for i in 0..n {
        let (a, b) = ab[i];

        if a <= s {
            for j in a..=s {
                if dp[i][j - a].is_some() {
                    dp[i + 1][j] = Some('H');
                }
            }
        }

        if b <= s {
            for j in b..=s {
                if dp[i][j - b].is_some() {
                    dp[i + 1][j] = Some('T');
                }
            }
        }
    }

    dp[n][s]?;

    let mut ans = vec![];
    let mut rem = s;

    for i in (0..n).rev() {
        let (a, b) = ab[i];
        let c = dp[i + 1][rem].unwrap();

        if c == 'H' {
            ans.push('H');
            rem -= a;
        } else {
            ans.push('T');
            rem -= b;
        }
    }

    Some(ans.into_iter().rev().collect())
}
