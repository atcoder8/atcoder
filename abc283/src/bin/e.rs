use proconio::input;

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<usize> {
    input! {
        (h, w): (usize, usize),
        mut aaa: [[usize; w]; h],
    }

    let check_isolated = |aaa: &Vec<Vec<usize>>, i: usize, j: usize, rev_flags: usize| {
        let prev_rev_flag = rev_flags & 1;
        let cur_rev_flag = (rev_flags >> 1) & 1;
        let next_rev_flag = (rev_flags >> 2) & 1;

        let cur_elem = aaa[i][j] ^ cur_rev_flag;

        if i > 0 && aaa[i - 1][j] ^ prev_rev_flag == cur_elem {
            return false;
        }

        if i < h - 1 && aaa[i + 1][j] ^ next_rev_flag == cur_elem {
            return false;
        }

        if j > 0 && aaa[i][j - 1] ^ cur_rev_flag == cur_elem {
            return false;
        }

        if j < w - 1 && aaa[i][j + 1] ^ cur_rev_flag == cur_elem {
            return false;
        }

        true
    };

    let check_any_isolated = |aaa: &Vec<Vec<usize>>, i: usize, rev_flags: usize| {
        (0..w).any(|j| check_isolated(aaa, i, j, rev_flags))
    };

    let mut dp = vec![None; 8];
    dp[0] = Some(0);
    dp[4] = Some(0);

    for i in 0..h {
        let mut next_dp: Vec<Option<usize>> = vec![None; 8];

        for rev_flags in 0..8 {
            if check_any_isolated(&aaa, i, rev_flags) {
                continue;
            }

            let cur_rev_flag = (rev_flags >> 1) & 1;

            if let Some(prev_cnt) = dp[(rev_flags << 1) & 7] {
                let next_cnt = prev_cnt + cur_rev_flag;

                if next_dp[rev_flags].is_none() {
                    next_dp[rev_flags] = Some(next_cnt);
                } else {
                    chmin!(next_dp[rev_flags], Some(next_cnt));
                }
            }

            if let Some(prev_cnt) = dp[((rev_flags << 1) & 7) | 1] {
                let next_cnt = prev_cnt + cur_rev_flag;

                if next_dp[rev_flags].is_none() {
                    next_dp[rev_flags] = Some(next_cnt);
                } else {
                    chmin!(next_dp[rev_flags], Some(next_cnt));
                }
            }
        }

        dp = next_dp;
    }

    dp.iter().filter_map(|x| *x).min()
}

/// If the right-hand side is less than the left-hand side,
/// clones the right-hand side, bind it to the left-hand side,
/// and return `true`.
/// If the right-hand side is greater than or equal to the left-hand side,
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
/// assert_eq!(chmin!(x, 7), false);
/// assert_eq!(x, 5);
///
/// assert_eq!(chmin!(x, 3), true);
/// assert_eq!(x, 3);
/// ```
#[macro_export]
macro_rules! chmin {
    ($lhs: expr, $rhs: expr) => {
        if $rhs < $lhs {
            let temp = $rhs.clone();
            $lhs = temp;
            true
        } else {
            false
        }
    };
}
