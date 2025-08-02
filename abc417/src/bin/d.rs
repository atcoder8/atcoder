use itertools::{enumerate, Itertools};
use ndarray::prelude::*;
use proconio::input;
use superslice::Ext;

const MAX: usize = 1000;

fn main() {
    input! {
        n: usize,
        pab: [(usize, usize, usize); n],
        q: usize,
        xx: [usize; q],
    }

    let mut prefix_sum_b = vec![0_usize; n + 1];
    for (i, &(_, _, b)) in enumerate(&pab) {
        prefix_sum_b[i + 1] = prefix_sum_b[i] + b;
    }

    let mut tension_array = Array2::from_elem((n + 1, MAX + 1), 0);
    for i in 0..=MAX {
        tension_array[(n, i)] = i;
    }
    for (i, &(p, a, b)) in enumerate(&pab).rev() {
        for tension in 0..=MAX {
            tension_array[(i, tension)] = if p >= tension {
                tension_array[(i + 1, tension + a)]
            } else {
                tension_array[(i + 1, tension.saturating_sub(b))]
            };
        }
    }

    let solve = |x: usize| {
        if x <= MAX {
            return tension_array[(0, x)];
        }

        let start_position = prefix_sum_b.lower_bound(&(x - MAX));

        if start_position == n + 1 {
            return x.saturating_sub(prefix_sum_b[n]);
        }

        tension_array[(
            start_position,
            x.saturating_sub(prefix_sum_b[start_position]),
        )]
    };

    println!("{}", xx.iter().map(|&x| solve(x)).join("\n"));
}
