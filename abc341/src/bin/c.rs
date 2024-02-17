use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w, _n): (usize, usize, usize),
        t: Chars,
        ss: [Chars; h],
    }

    let sea_mat = Array2::from_shape_fn((h, w), |(row, col)| ss[row][col] == '#');

    let mut dp = Array2::from_shape_fn((h, w), |(row, col)| ss[row][col] == '.');
    let mut ans = dp.iter().filter(|&&x| x).count();
    let mut next_dp = Array2::from_elem((h, w), false);
    for &c in &t {
        let (diff_row, diff_col) = match c {
            'U' => DIFFS[0],
            'L' => DIFFS[1],
            'R' => DIFFS[2],
            'D' => DIFFS[3],
            _ => panic!(),
        };

        for (row, col) in iproduct!(1..h - 1, 1..w - 1) {
            if !dp[(row, col)] {
                continue;
            }

            let next_row = row.wrapping_add(diff_row);
            let next_col = col.wrapping_add(diff_col);
            if sea_mat[(next_row, next_col)] {
                ans -= 1;
            } else {
                next_dp[(next_row, next_col)] = true;
            }
        }

        std::mem::swap(&mut dp, &mut next_dp);
        next_dp.fill(false);
    }

    println!("{}", ans);
}
