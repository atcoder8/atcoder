use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        cc: [usize; n],
    }

    let (cost0, cost1) = if s[0] == '0' { (0, cc[0]) } else { (cc[0], 0) };

    let mut bad_zero: Option<usize> = Some(cost0);
    let mut bad_one: Option<usize> = Some(cost1);
    let mut good_zero: Option<usize> = None;
    let mut good_one: Option<usize> = None;

    for i in 1..n {
        let mut next_bad_zero: Option<usize> = None;
        let mut next_bad_one: Option<usize> = None;
        let mut next_good_zero: Option<usize> = None;
        let mut next_good_one: Option<usize> = None;

        let (cost0, cost1) = if s[i] == '0' { (0, cc[i]) } else { (cc[i], 0) };

        chmin_for_option(&mut next_bad_zero, bad_one.unwrap() + cost0);

        chmin_for_option(&mut next_bad_one, bad_zero.unwrap() + cost1);

        chmin_for_option(&mut next_good_zero, bad_zero.unwrap() + cost0);
        if let Some(same_one) = good_one {
            chmin_for_option(&mut next_good_zero, same_one + cost0);
        }

        chmin_for_option(&mut next_good_one, bad_one.unwrap() + cost1);
        if let Some(same_zero) = good_zero {
            chmin_for_option(&mut next_good_one, same_zero + cost1);
        }

        bad_zero = next_bad_zero;
        bad_one = next_bad_one;
        good_zero = next_good_zero;
        good_one = next_good_one;
    }

    let ans = good_zero.unwrap().min(good_one.unwrap());
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
