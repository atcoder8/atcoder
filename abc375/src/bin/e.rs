use ndarray::Array2;
use proconio::{input, marker::Usize1};

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        ab: [(Usize1, usize); n],
    }

    let sum_b = ab.iter().map(|&(_, b)| b).sum::<usize>();

    if sum_b % 3 != 0 {
        return None;
    }

    let strength_per_team = sum_b / 3;

    let mut dp =
        Array2::<Option<usize>>::from_elem((strength_per_team + 1, strength_per_team + 1), None);
    dp[(0, 0)] = Some(0);
    let mut next_dp =
        Array2::<Option<usize>>::from_elem((strength_per_team + 1, strength_per_team + 1), None);
    let mut acc = 0_usize;
    for &(a, b) in &ab {
        for ((s1, s2), &cost) in dp.indexed_iter() {
            let Some(cost) = cost else {
                continue;
            };

            // チーム1に所属させる
            if s1 + b <= strength_per_team {
                chmin_for_option(&mut next_dp[(s1 + b, s2)], cost + (a != 0) as usize);
            }

            // チーム2に所属させる
            if s2 + b <= strength_per_team {
                chmin_for_option(&mut next_dp[(s1, s2 + b)], cost + (a != 1) as usize);
            }

            // チーム3に所属させる
            if acc - (s1 + s2) + b <= strength_per_team {
                chmin_for_option(&mut next_dp[(s1, s2)], cost + (a != 2) as usize);
            }
        }

        acc += b;
        std::mem::swap(&mut dp, &mut next_dp);
        next_dp.fill(None);
    }

    dp[(strength_per_team, strength_per_team)]
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
