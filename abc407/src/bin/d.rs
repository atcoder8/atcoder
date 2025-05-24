use itertools::izip;
use ndarray::prelude::*;
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [[usize; w]; h],
    }

    let value_array = Array2::from_shape_fn((h, w), |(row, col)| aaa[row][col]);
    let max_score = recursion(&value_array, &Array2::from_elem((h, w), false), [0; 2]);
    println!("{}", max_score);
}

fn recursion(
    value_array: &Array2<usize>,
    filled_array: &Array2<bool>,
    curr_coord: [usize; 2],
) -> usize {
    let shape = value_array.shape();
    let (h, w) = (shape[0], shape[1]);

    if curr_coord == [h, 0] {
        return calc_score(value_array, filled_array);
    }

    let [row, col] = curr_coord;

    let mut scores = vec![];
    if col + 1 < w && !filled_array[[row, col]] && !filled_array[[row, col + 1]] {
        let mut next_filled_array = filled_array.clone();
        next_filled_array[[row, col]] = true;
        next_filled_array[[row, col + 1]] = true;
        let score = recursion(value_array, &next_filled_array, curr_coord);
        scores.push(score);
    }
    if row + 1 < h && !filled_array[[row, col]] && !filled_array[[row + 1, col]] {
        let mut next_filled_array = filled_array.clone();
        next_filled_array[[row, col]] = true;
        next_filled_array[[row + 1, col]] = true;
        let score = recursion(value_array, &next_filled_array, curr_coord);
        scores.push(score);
    }
    let [mut next_row, mut next_col] = curr_coord;
    next_col += 1;
    if next_col == w {
        (next_row, next_col) = (next_row + 1, 0);
    }
    scores.push(recursion(value_array, filled_array, [next_row, next_col]));

    *scores.iter().max().unwrap()
}

fn calc_score(score_array: &Array2<usize>, filled_array: &Array2<bool>) -> usize {
    izip!(score_array, filled_array).fold(0_usize, |acc, (&value, &filled)| {
        acc ^ (value * !filled as usize)
    })
}
