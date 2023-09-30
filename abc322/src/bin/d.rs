use itertools::Itertools;
use proconio::{input, marker::Chars};

const LEN: usize = 4;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        ps: [[Chars; LEN]; 3],
    }

    let mut ps2 = vec![];
    for p in ps {
        let mut poly = vec![vec![false; LEN]; LEN];
        for (i, j) in (0..LEN).cartesian_product(0..LEN) {
            poly[i][j] = p[i][j] == '#';
        }

        ps2.push(poly);
    }

    let mut poly1 = justified(&ps2[0]);
    let mut poly2 = justified(&ps2[1]);
    let mut poly3 = justified(&ps2[2]);

    for bit in 0..64 {
        for (shift_row_1, shift_col_1) in (0..LEN).cartesian_product(0..LEN) {
            let shifted_poly_1 = match shifted(&poly1, shift_row_1, shift_col_1) {
                Some(x) => x,
                None => continue,
            };

            for (shift_row_2, shift_col_2) in (0..LEN).cartesian_product(0..LEN) {
                let shifted_poly_2 = match shifted(&poly2, shift_row_2, shift_col_2) {
                    Some(x) => x,
                    None => continue,
                };

                for (shift_row_3, shift_col_3) in (0..LEN).cartesian_product(0..LEN) {
                    let shifted_poly_3 = match shifted(&poly3, shift_row_3, shift_col_3) {
                        Some(x) => x,
                        None => continue,
                    };

                    if can_fill(&vec![&shifted_poly_1, &shifted_poly_2, &shifted_poly_3]) {
                        return true;
                    }
                }
            }
        }

        poly1 = justified(&rotated(&poly1));

        if bit % 4 == 0 {
            poly2 = justified(&rotated(&poly2));
        }

        if bit % 16 == 0 {
            poly3 = justified(&rotated(&poly3));
        }
    }

    false
}

fn justified(poly: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let min_row = poly.iter().position(|pp| pp.iter().any(|&p| p)).unwrap();
    let min_col = (0..LEN)
        .position(|col| (0..LEN).any(|row| poly[row][col]))
        .unwrap();

    let mut justified_poly = vec![vec![false; LEN]; LEN];
    for (i, j) in (0..(LEN - min_row)).cartesian_product(0..(LEN - min_col)) {
        justified_poly[i][j] = poly[i + min_row][j + min_col];
    }

    justified_poly
}

fn rotated(poly: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut rotated_poly = vec![vec![false; LEN]; LEN];
    for (i, j) in (0..LEN).cartesian_product(0..LEN) {
        rotated_poly[i][j] = poly[LEN - 1 - j][i];
    }

    rotated_poly
}

fn shifted(poly: &[Vec<bool>], shift_row: usize, shift_col: usize) -> Option<Vec<Vec<bool>>> {
    let mut shifted_poly = vec![vec![false; LEN]; LEN];
    for (i, j) in (0..LEN).cartesian_product(0..LEN) {
        if i + shift_row < LEN && j + shift_col < LEN {
            shifted_poly[i + shift_row][j + shift_col] = poly[i][j];
        } else if poly[i][j] {
            return None;
        }
    }

    Some(shifted_poly)
}

fn can_fill(ps: &[&Vec<Vec<bool>>]) -> bool {
    let mut filled_poly = vec![vec![false; LEN]; LEN];
    for poly in ps {
        for (i, j) in (0..LEN).cartesian_product(0..LEN) {
            if poly[i][j] {
                if filled_poly[i][j] {
                    return false;
                }

                filled_poly[i][j] = true;
            }
        }
    }

    for (i, j) in (0..LEN).cartesian_product(0..LEN) {
        if !filled_poly[i][j] {
            return false;
        }
    }

    true
}
