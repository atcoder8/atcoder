use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    let mut x = 0;
    let mut y = 0;
    let mut grid: Vec<Vec<String>> = vec![vec![String::from("T"); n]; n];
    let mut dir = Dir::Right;
    for label in 1..=n.pow(2) - 1 {
        grid[x][y] = label.to_string();

        match dir {
            Dir::Up => {
                x -= 1;

                if x == 0 || grid[x - 1][y] != "T" {
                    dir = Dir::Right;
                }
            }

            Dir::Down => {
                x += 1;

                if x == n - 1 || grid[x + 1][y] != "T" {
                    dir = Dir::Left;
                }
            }

            Dir::Left => {
                y -= 1;

                if y == 0 || grid[x][y - 1] != "T" {
                    dir = Dir::Up;
                }
            }

            Dir::Right => {
                y += 1;

                if y == n - 1 || grid[x][y + 1] != "T" {
                    dir = Dir::Down;
                }
            }
        }
    }

    for grid_row in &grid {
        println!("{}", grid_row.iter().join(" "));
    }
}
