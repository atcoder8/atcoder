use itertools::iproduct;
use ndarray::prelude::*;
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [[usize; w]; h],
        pp: [usize; h + w - 1],
    }

    let mut require_coin_array = Array2::from_elem((h, w), None::<usize>);
    require_coin_array[(h - 1, w - 1)] = Some(0);
    for (row, col) in iproduct!((0..h).rev(), (0..w).rev()) {
        let subsequent_require_coin = require_coin_array[(row, col)].unwrap();
        let require_coin = (pp[row + col] + subsequent_require_coin).saturating_sub(aaa[row][col]);
        if row > 0 {
            chmin_for_option(&mut require_coin_array[(row - 1, col)], require_coin);
        }
        if col > 0 {
            chmin_for_option(&mut require_coin_array[(row, col - 1)], require_coin);
        }
    }

    let require_coin = (pp[0] + require_coin_array[(0, 0)].unwrap()).saturating_sub(aaa[0][0]);
    println!("{}", require_coin);
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
