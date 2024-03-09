use std::cmp::Reverse;

use itertools::iproduct;
use ndarray::Array2;
use proconio::input;

const INF: usize = 10_usize.pow(12);

fn main() {
    input! {
        n: usize,
        ppp: [[usize; n]; n],
        hor_cost: [[usize; n - 1]; n],
        ver_cost: [[usize; n]; n - 1],
    }

    let create_cost_mat = |to_coord: (usize, usize)| {
        let mut mat = Array2::<usize>::from_elem((n, n), INF);
        mat[to_coord] = 0;

        for from_row in (0..=to_coord.0).rev() {
            for from_col in (0..=to_coord.1).rev() {
                let from_coord = (from_row, from_col);

                if from_row < to_coord.0 {
                    mat[from_coord] = mat[from_coord]
                        .min(mat[(from_row + 1, from_col)] + ver_cost[from_row][from_col]);
                }

                if from_col < to_coord.1 {
                    mat[from_coord] = mat[from_coord]
                        .min(mat[(from_row, from_col + 1)] + hor_cost[from_row][from_col]);
                }
            }
        }

        mat
    };

    let mut dp = Array2::<(usize, usize)>::from_elem((n, n), (0, 0));
    dp[(0, 0)] = (0, 0);
    for to_coord in iproduct!(0..n, 0..n).skip(1) {
        let cost_mat = create_cost_mat(to_coord);

        dp[to_coord] = iproduct!(0..=to_coord.0, 0..=to_coord.1)
            .filter(|&from_coord| from_coord != to_coord)
            .map(|from_coord| {
                let dist = to_coord.0 - from_coord.0 + to_coord.1 - from_coord.1;
                let (min_op_num, money) = dp[from_coord];
                let cost = cost_mat[from_coord];

                if money >= cost {
                    return (min_op_num + dist, money - cost);
                }

                let rem_cost = cost - money;
                let p = ppp[from_coord.0][from_coord.1];
                let stay_time = (rem_cost + p - 1) / p;

                (min_op_num + dist + stay_time, p * stay_time - rem_cost)
            })
            .min_by_key(|&(op_num, money)| (op_num, Reverse(money)))
            .unwrap();
    }

    println!("{}", dp[(n - 1, n - 1)].0);
}
