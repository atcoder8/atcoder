use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    match solve(&aa) {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve(aa: &[usize]) -> Option<usize> {
    let mut min_length = None::<usize>;
    let max_a = *aa.iter().max().unwrap();
    let mut positions = vec![None::<usize>; max_a + 1];
    for (i, &a) in enumerate(aa) {
        if let Some(prev) = positions[a].take() {
            chmin_for_option(&mut min_length, i - prev + 1);
        }
        positions[a] = Some(i);
    }

    min_length
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
