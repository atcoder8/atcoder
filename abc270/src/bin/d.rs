use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; k],
    }

    let mut dp = vec![0; n + 1];

    for i in 1..=n {
        for &a in &aa {
            if a <= i {
                chmax!(dp[i], i - dp[i - a]);
            }
        }
    }

    println!("{}", dp[n]);
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
