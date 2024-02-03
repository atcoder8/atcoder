use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
    }

    enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    let mut grid = vec![vec![false; w]; h];
    let mut row = 0;
    let mut col = 0;
    let mut dir = Dir::Up;
    for _ in 0..n {
        grid[row][col] = !grid[row][col];

        dir = if grid[row][col] {
            match dir {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            }
        } else {
            match dir {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
            }
        };

        match dir {
            Dir::Up => row = (row + h - 1) % h,
            Dir::Down => row = (row + 1) % h,
            Dir::Left => col = (col + w - 1) % w,
            Dir::Right => col = (col + 1) % w,
        }
    }

    println!(
        "{}",
        grid.iter()
            .map(|line| line
                .iter()
                .map(|&cell| if cell { '#' } else { '.' })
                .join(""))
            .join("\n")
    );
}
