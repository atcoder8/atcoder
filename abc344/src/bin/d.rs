use proconio::input;

fn main() {
    input! {
        t: String,
        bags: [[String]],
    }

    let mut dp: Vec<Option<usize>> = vec![None; t.len() + 1];
    dp[0] = Some(0);

    for strings in bags {
        let mut next_dp = dp.clone();

        for s in strings {
            if s.len() > t.len() {
                continue;
            }

            for len in 0..=t.len() - s.len() {
                if &t[len..len + s.len()] == &s {
                    if let Some(cost) = dp[len] {
                        chmin_for_option(&mut next_dp[len + s.len()], cost + 1);
                    }
                }
            }
        }

        dp = next_dp;
    }

    match dp[t.len()] {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
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
