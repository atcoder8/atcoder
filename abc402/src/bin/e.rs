use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
    }

    let problems = (0..n).map(|i| Problem::read(i)).collect_vec();

    let mut dp = Array2::from_elem((x + 1, 1 << n), 0.0_f64);
    for (rem, bits) in iproduct!(0..=x, 0..1 << n) {
        let calc_exp = |problem: &Problem| {
            if problem.cost > rem || bits >> problem.id & 1 == 1 {
                return 0.0;
            }

            let next_rem = rem - problem.cost;
            let collect_exp = dp[(next_rem, bits | (1 << problem.id))] + problem.score;
            let wrong_exp = dp[(next_rem, bits)];
            collect_exp * problem.probability + wrong_exp * (1.0 - problem.probability)
        };

        dp[(rem, bits)] = problems
            .iter()
            .map(calc_exp)
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
    }

    println!("{}", dp[(x, 0)]);
}

#[derive(Debug, Clone, Copy)]
struct Problem {
    id: usize,
    score: f64,
    cost: usize,
    probability: f64,
}

impl Problem {
    fn read(id: usize) -> Self {
        input! {
            (s, c, p): (f64, usize, f64),
        }

        Self {
            id,
            score: s,
            cost: c,
            probability: p / 100.0,
        }
    }
}
