use proconio::input;

fn main() {
    input! {
        (n, k, d): (usize, usize, usize),
        aa: [usize; n],
    }

    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; k + 1]; d];
    dp[0][0] = Some(0);

    for &a in &aa {
        let mut next_dp = dp.clone();

        for cur_rem in 0..d {
            for cur_size in 0..k {
                if let Some(cur_val) = dp[cur_rem][cur_size] {
                    chmax!(next_dp[(cur_rem + a) % d][cur_size + 1], Some(cur_val + a));
                }
            }
        }

        dp = next_dp;
    }

    if let Some(ans) = dp[0][k] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

/// If the right-hand side is greater than the left-hand side,
/// clones the right-hand side, bind it to the left-hand side,
/// and return `true`.
/// If the right-hand side is less than or equal to the left-hand side,
/// does nothing and returns `false`.
///
/// The left-hand and right-hand sides must be the same type
/// and must implement the `Clone` trait.
///
/// # Examples
///
/// ```
/// let mut x = 5;
///
/// assert_eq!(chmax!(x, 3), false);
/// assert_eq!(x, 5);
///
/// assert_eq!(chmax!(x, 7), true);
/// assert_eq!(x, 7);
/// ```
#[macro_export]
macro_rules! chmax {
    ($lhs: expr, $rhs: expr) => {
        if $rhs > $lhs {
            let temp = $rhs.clone();
            $lhs = temp;
            true
        } else {
            false
        }
    };
}
