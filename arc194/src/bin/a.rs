use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut dp = vec![None::<i64>; n + 1];
    dp[0] = Some(0);
    for (i, &a) in enumerate(&aa) {
        let acc = dp[i].unwrap();
        chmax_for_option(&mut dp[i + 1], acc + a);
        if i + 2 <= n {
            chmax_for_option(&mut dp[i + 2], acc);
        }
    }

    println!("{}", dp[n].unwrap());
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
