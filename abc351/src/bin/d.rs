use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let mut ans = 0;
    let mut labels = Array2::<Option<usize>>::from_elem((h, w), None);
    let mut label = 0;

    for start_coord in iproduct!(0..h, 0..w) {
        if ss[start_coord.0][start_coord.1] == '#' || labels[start_coord].is_some() {
            continue;
        }

        let mut stack = vec![start_coord];
        let mut area = 0;

        while let Some(coord) = stack.pop() {
            let (row, col) = coord;

            if labels[coord] == Some(label) {
                continue;
            }

            labels[coord] = Some(label);

            area += 1;

            let mut adj_coords = vec![];
            for (diff_row, diff_col) in DIFFS {
                let next_row = row.wrapping_add(diff_row);
                let next_col = col.wrapping_add(diff_col);

                if next_row < h && next_col < w {
                    adj_coords.push((next_row, next_col));
                }
            }

            if adj_coords
                .iter()
                .all(|&(adj_row, adj_col)| ss[adj_row][adj_col] != '#')
            {
                stack.extend(adj_coords);
            }
        }

        ans = ans.max(area);

        label += 1;
    }

    println!("{}", ans);
}
