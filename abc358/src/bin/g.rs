use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Usize1};

const DIFFS: [(usize, usize); 5] = [(!0, 0), (0, !0), (0, 0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w, k): (usize, usize, usize),
        init_coord: (Usize1, Usize1),
        aaa: [usize; h * w],
    }

    let grid = Array2::from_shape_vec((h, w), aaa).unwrap();

    let mut dp = Array2::from_elem((h, w), None::<usize>);
    dp[init_coord] = Some(0);
    let mut next_dp = Array2::from_elem((h, w), None::<usize>);
    let simulation_turn = (h * w).min(k);

    for _ in 0..(h * w).min(k) {
        for (row, col) in iproduct!(0..h, 0..w) {
            let Some(score) = dp[(row, col)] else { continue; };

            for (diff_row, diff_col) in DIFFS {
                let next_row = row.wrapping_add(diff_row);
                let next_col = col.wrapping_add(diff_col);
                let next_coord = (next_row, next_col);

                if next_row < h && next_col < w {
                    chmax_for_option(&mut next_dp[next_coord], score + grid[next_coord]);
                }
            }
        }

        std::mem::swap(&mut dp, &mut next_dp);
        next_dp.fill(None);
    }

    let ans = dp
        .indexed_iter()
        .filter_map(|(coord, &score)| Some(score? + grid[coord] * (k - simulation_turn)))
        .max()
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
