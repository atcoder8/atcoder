use itertools::iproduct;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let is_ok = |coord: (usize, usize)| {
        let (row, col) = coord;
        if ss[row][col] != '#' {
            return true;
        }
        let num_black_squares = DIFFS
            .iter()
            .filter(|&&(dr, dc)| {
                let adj_row = row.wrapping_add(dr);
                let adj_col = col.wrapping_add(dc);
                adj_row < h && adj_col < w && ss[adj_row][adj_col] == '#'
            })
            .count();
        [2, 4].contains(&num_black_squares)
    };

    let ans = iproduct!(0..h, 0..w).all(|coord| is_ok(coord));
    println!("{}", if ans { "Yes" } else { "No" });
}
