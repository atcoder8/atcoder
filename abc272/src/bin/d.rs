use std::collections::VecDeque;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let mut grid = vec![vec![n * n; n]; n];
    grid[0][0] = 0;

    let mut que = VecDeque::from(vec![(0, 0)]);

    while let Some((cur_x, cur_y)) = que.pop_front() {
        let next_cost = grid[cur_x][cur_y] + 1;

        for diff_x in (0..).take_while(|&i| i * i <= m) {
            let sq_diff_y = m - diff_x * diff_x;
            let diff_y = sq_diff_y.sqrt();

            if diff_y * diff_y != sq_diff_y {
                continue;
            }

            let next_xs = vec![cur_x.checked_sub(diff_x), cur_x.checked_add(diff_x)];
            let next_ys = vec![cur_y.checked_sub(diff_y), cur_y.checked_add(diff_y)];

            for &next_x in &next_xs {
                for &next_y in &next_ys {
                    if let (Some(next_x), Some(next_y)) = (next_x, next_y) {
                        if next_x >= n || next_y >= n {
                            continue;
                        }

                        if grid[next_x][next_y] == n * n {
                            grid[next_x][next_y] = next_cost;
                            que.push_back((next_x, next_y));
                        }
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] < n * n {
                print!("{} ", grid[i][j]);
            } else {
                print!("-1 ");
            }
        }
        println!();
    }
}
