use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [i64; n],
        lr: [(usize, usize); q],
    }

    let calc_prefix_sum = |degree: usize| {
        (0..=n)
            .scan(0_i64, |acc, k| {
                if k == 0 {
                    return Some(0);
                }

                *acc += aa[k - 1] * k.pow(degree as u32) as i64;
                Some(*acc)
            })
            .collect_vec()
    };

    let prefix_sum_by_degree: [Vec<i64>; 3] = std::array::from_fn(|degree| calc_prefix_sum(degree));

    let calc_sum = |degree: usize, l: usize, r: usize| {
        let prefix_sum = &prefix_sum_by_degree[degree];
        prefix_sum[r] - prefix_sum[l - 1]
    };

    let solve = |l: usize, r: usize| {
        let coefficients = {
            let (l, r) = (l as i64, r as i64);
            [(-l + 1) * (r + 1), l + r, -1]
        };
        enumerate(coefficients)
            .map(|(degree, coefficient)| coefficient * calc_sum(degree, l, r))
            .sum::<i64>()
    };

    let output = lr.iter().map(|&(l, r)| solve(l, r)).join("\n");
    println!("{output}");
}
