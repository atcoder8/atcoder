use itertools::{enumerate, Itertools};
use ndarray::Array2;
use proconio::input;

fn main() {
    input! {
        (n, m, l): (usize, usize, usize),
        aa: [usize; n],
    }

    let calc_cost = |offset: usize, dest: usize| {
        aa[offset..]
            .iter()
            .step_by(l)
            .map(|&a| (dest + m - a) % m)
            .sum::<usize>()
    };

    let cost_array = Array2::from_shape_fn((l, m), |(offset, dest)| calc_cost(offset, dest));

    let init_costs = (0..m).map(|dest| cost_array[(0, dest)]).collect_vec();
    let costs_by_rem = (1..l).fold(init_costs, |costs, offset| {
        (0..m)
            .map(|to| {
                enumerate(&costs)
                    .map(|(from, &cost)| cost + cost_array[(offset, (to + m - from) % m)])
                    .min()
                    .unwrap()
            })
            .collect()
    });

    println!("{}", costs_by_rem[0]);
}
