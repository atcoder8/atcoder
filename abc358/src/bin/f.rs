use itertools::{iproduct, Itertools};
use ndarray::{Array2, Axis};
use proconio::input;

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    match solve() {
        Some(maze) => {
            let maze = maze
                .axis_iter(Axis(0))
                .map(|line| line.iter().collect::<String>())
                .join("\n");
            println!("Yes\n{}", maze);
        }
        None => println!("No"),
    }
}

fn solve() -> Option<Array2<char>> {
    input! {
        (n, m, k): (usize, usize, usize),
    }

    if k < n || (k - n) % 2 == 1 {
        return None;
    }

    let mut route = Array2::from_elem((n, m), false);
    let mut redundant = k - n;
    for row in (0..n / 2 * 2).step_by(2) {
        let extend = (redundant / 2).min(m - 1);
        for i in 0..extend + 1 {
            for offset in 0..2 {
                route[(row + offset, m - 1 - i)] = true;
            }
        }

        redundant -= 2 * extend;
    }

    for col in 0..redundant {
        route[(n - 1, col)] = true;
    }

    route[(n - 1, m - 1)] = true;

    let mut orders = Array2::from_shape_fn((n, m), |(row, col)| {
        row * m + if row % 2 == 0 { m - 1 - col } else { col }
    });
    if n % 2 == 1 {
        for col in 0..m {
            for offset in 0..2 {
                orders[(n - 2 + offset, col)] = (n - 2) * m + 2 * col + (col % 2 ^ offset);
            }
        }
    }

    let mut dists = Array2::<Option<usize>>::from_elem((n, m), None);
    let mut cur_coord: (usize, usize) = (0, m - 1);
    dists[cur_coord] = Some(0);

    loop {
        let (row, col) = cur_coord;

        let next_coord = DIFFS
            .iter()
            .filter_map(|&(diff_row, diff_col)| {
                let next_row = row.wrapping_add(diff_row);
                let next_col = col.wrapping_add(diff_col);
                let next_coord = (next_row, next_col);

                if next_row < n && next_col < m && route[next_coord] && dists[next_coord].is_none()
                {
                    Some(next_coord)
                } else {
                    None
                }
            })
            .min_by_key(|&next_coord| orders[next_coord])
            .unwrap();
        dists[next_coord] = Some(dists[cur_coord].unwrap() + 1);

        cur_coord = next_coord;

        if cur_coord == (n - 1, m - 1) {
            break;
        }
    }

    let init_symbol = |coord: (usize, usize)| match coord {
        (row, col) if row == 0 && col == 2 * m - 1 => 'S',
        (row, col) if row == 2 * n && col == 2 * m - 1 => 'G',
        (row, col) if row == 0 || row == 2 * n || col == 0 || col == 2 * m => '+',
        (row, col) if row % 2 == 0 && col % 2 == 0 => '+',
        (row, col) if row % 2 == 1 && col % 2 == 1 => 'o',
        _ => '.',
    };

    let mut maze = Array2::from_shape_fn((2 * n + 1, 2 * m + 1), init_symbol);
    for (row, col) in iproduct!(0..n, 0..m) {
        if row < n - 1 {
            match (dists[(row, col)], dists[(row + 1, col)]) {
                (Some(dist1), Some(dist2)) if dist1.abs_diff(dist2) == 1 => {}
                _ => {
                    maze[(2 * row + 2, 2 * col + 1)] = '-';
                }
            }
        }

        if col < m - 1 {
            match (dists[(row, col)], dists[(row, col + 1)]) {
                (Some(dist1), Some(dist2)) if dist1.abs_diff(dist2) == 1 => {}
                _ => {
                    maze[(2 * row + 1, 2 * col + 2)] = '|';
                }
            }
        }
    }

    Some(maze)
}
