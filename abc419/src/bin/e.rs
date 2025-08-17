use itertools::{enumerate, Itertools};
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

    let init_costs = (0..m).map(|dest| calc_cost(0, dest)).collect_vec();
    let costs_by_rem = (1..l).fold(init_costs, |costs, offset| {
        (0..m)
            .map(|to| {
                enumerate(&costs)
                    .map(|(from, &cost)| cost + calc_cost(offset, (to + m - from) % m))
                    .min()
                    .unwrap()
            })
            .collect()
    });

    println!("{}", costs_by_rem[0]);
}
