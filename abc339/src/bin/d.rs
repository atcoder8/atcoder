use std::collections::{BTreeSet, VecDeque};

use itertools::{iproduct, Itertools};
use proconio::{input, marker::Chars};

const DIFFS: [(i64, i64); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let (init_coord_1, init_coord_2) = iproduct!(0..n, 0..n)
        .filter(|&(row, col)| ss[row][col] == 'P')
        .collect_tuple()
        .unwrap();

    let moved = |row: usize, col: usize, diff_row: i64, diff_col: i64| {
        let next_row = (row as i64 + diff_row).clamp(0, n as i64 - 1) as usize;
        let next_col = (col as i64 + diff_col).clamp(0, n as i64 - 1) as usize;

        if ss[next_row][next_col] == '#' {
            (row, col)
        } else {
            (next_row, next_col)
        }
    };

    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::from([(init_coord_1, init_coord_2, 0)]);
    while let Some(((row1, col1), (row2, col2), dist)) = queue.pop_front() {
        if ss[row1][col1] == '#'
            || ss[row2][col2] == '#'
            || visited.contains(&((row1, col1), (row2, col2)))
        {
            continue;
        }

        if row1 == row2 && col1 == col2 {
            return Some(dist);
        }

        visited.insert(((row1, col1), (row2, col2)));

        for (diff_row, diff_col) in DIFFS {
            let moved_coord_1 = moved(row1, col1, diff_row, diff_col);
            let moved_coord_2 = moved(row2, col2, diff_row, diff_col);

            queue.push_back((moved_coord_1, moved_coord_2, dist + 1));
        }
    }

    None
}
