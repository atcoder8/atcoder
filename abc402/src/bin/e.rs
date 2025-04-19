// unfinished

use itertools::enumerate;
use ndarray::Array2;
use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        scp: [(f64, usize, f64); n],
    }

    // dp[(i,j)]: 残金iで解いた問題の集合がjの場合の最大の期待値
    let mut dp = Array2::from_elem((x + 1, 1_usize << n), None);
    dp[(x, 0)] = Some(0.0);
    for rem in (0..=x).rev() {
        for bits in 0..1_usize << n {
            let Some(max_exp) = dp[(rem, bits)] else {
                continue;
            };

            for (i, &(score, cost, p)) in enumerate(&scp) {
                let probably = p / 100.0;

                if bits >> i & 1 == 1 || cost > rem {
                    continue;
                }

                chmax_for_option(
                    &mut dp[(rem - cost, bits | (1 << i))],
                    (max_exp + score) * probably,
                );

                chmax_for_option(&mut dp[(rem - cost, bits)], max_exp);
            }
        }
    }

    let ans = dp
        .iter()
        .filter_map(|&max_exp| max_exp)
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    println!("{}", ans);
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
