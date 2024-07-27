use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (h, w): (usize, usize),
        start_coord: (Usize1, Usize1),
        ccc: [Chars; h],
        xx: Chars,
    }

    let (mut row, mut col) = start_coord;
    for &x in &xx {
        let (diff_row, diff_col) = match x {
            'U' => (!0, 0),
            'D' => (1, 0),
            'L' => (0, !0),
            'R' => (0, 1),
            _ => panic!(),
        };

        let adj_row = row.wrapping_add(diff_row);
        let adj_col = col.wrapping_add(diff_col);

        if adj_row < h && adj_col < w && ccc[adj_row][adj_col] == '.' {
            (row, col) = (adj_row, adj_col);
        }
    }

    println!("{} {}", row + 1, col + 1);
}
