use std::collections::VecDeque;

use im_rc::HashSet;
use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (h, w): (usize, usize),
        aaa: [[usize; w]; h],
        bbb: [[usize; w]; h],
    }

    let mut used = HashSet::<Vec<Vec<usize>>>::new();
    let mut queue = VecDeque::from([(aaa, 0)]);
    while let Some((grid, cost)) = queue.pop_front() {
        if grid == bbb {
            return Some(cost);
        }

        if used.contains(&grid) {
            continue;
        }

        used.insert(grid.clone());

        for row in 0..h - 1 {
            let mut next_grid = grid.clone();
            next_grid.swap(row, row + 1);

            queue.push_back((next_grid, cost + 1));
        }

        for col in 0..w - 1 {
            let mut next_grid = grid.clone();
            for row in 0..h {
                let t = next_grid[row][col];
                next_grid[row][col] = next_grid[row][col + 1];
                next_grid[row][col + 1] = t;
            }

            queue.push_back((next_grid, cost + 1));
        }
    }

    None
}
