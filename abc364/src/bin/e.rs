use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (n, x, y): (usize, usize, usize),
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![None; x + 1]; n];
    dp[0][0] = Some(0_usize);

    for &(a, b) in &ab {
        let end_sum_a = (x + 1).saturating_sub(a);
        for (eaten_num, sum_a) in iproduct!((0..n - 1).rev(), (0..end_sum_a).rev()) {
            if let Some(min_sum_b) = dp[eaten_num][sum_a] {
                if min_sum_b + b <= y {
                    chmin_for_option(&mut dp[eaten_num + 1][sum_a + a], min_sum_b + b);
                }
            }
        }
    }

    let ans = (0..=n - 1)
        .rev()
        .find(|&middle_eaten_num| dp[middle_eaten_num].iter().any(Option::is_some))
        .unwrap()
        + 1;
    println!("{}", ans);
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
