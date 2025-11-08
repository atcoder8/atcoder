use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        whb: [(i64, i64, i64); n],
    }

    let sum_w = whb.iter().map(|v| v.0).sum::<i64>();

    let mut dp = vec![None::<i64>; 2 * sum_w as usize + 1];
    dp[sum_w as usize] = Some(0);
    let mut next_dp = vec![None::<i64>; 2 * sum_w as usize + 1];
    for &(w, h, b) in &whb {
        for (i, &score) in enumerate(&dp) {
            let Some(score) = score else {
                continue;
            };

            if i >= w as usize {
                chmax_for_option(&mut next_dp[i - w as usize], score + h);
            }

            if (i + w as usize) < dp.len() {
                chmax_for_option(&mut next_dp[i + w as usize], score + b);
            }
        }

        std::mem::swap(&mut dp, &mut next_dp);
        next_dp.fill(None);
    }

    let max_score = dp[sum_w as usize..]
        .iter()
        .filter_map(|&v| v)
        .max()
        .unwrap();
    println!("{max_score}");
}

/// If `value` is `None` or contains a value less than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmax_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost >= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
