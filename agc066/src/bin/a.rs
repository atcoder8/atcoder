use itertools::Itertools;
use ndarray::{Array, Axis};
use proconio::input;

fn main() {
    input! {
        (n, d): (usize, i64),
        aaa: [i64; n.pow(2)],
    }

    let cost_limit = d * n.pow(2) as i64 / 2;
    let d2 = 2 * d;

    let grid = Array::from_shape_vec((n, n), aaa).unwrap();

    let solve = |base: i64| {
        let rems = [base, (base + d) % d2];

        let mut updated_grid = Array::from_elem((n, n), 0_i64);
        let mut cost = 0;

        for (coord, &a) in grid.indexed_iter() {
            let rem = rems[(coord.0 + coord.1) % 2];

            let diff = ((a - rem) % d2 + d2) % d2;

            if diff <= d2 - diff {
                updated_grid[coord] = a - diff;
                cost += diff;
            } else {
                updated_grid[coord] = a + d2 - diff;
                cost += d2 - diff;
            }

            if cost > cost_limit {
                return None;
            }
        }

        Some(updated_grid)
    };

    let grid = solve(0).or(solve(d)).unwrap();
    let ans = grid
        .axis_iter(Axis(0))
        .map(|line| line.iter().join(" "))
        .join("\n");
    println!("{}", ans);
}
