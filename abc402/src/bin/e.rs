use itertools::{enumerate, Itertools};
use ndarray::prelude::*;
use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
    }

    let problems = (0..n).map(|_| Problem::read()).collect_vec();

    let ans = recursion(
        &problems,
        x,
        0,
        &mut Array2::from_elem((x + 1, 1 << n), None),
    );
    println!("{}", ans);
}

#[derive(Debug, Clone, Copy)]
struct Problem {
    score: usize,
    cost: usize,
    probability: f64,
}

impl Problem {
    fn read() -> Self {
        input! {
            (s, c, p): (usize, usize, f64),
        }

        Self {
            score: s,
            cost: c,
            probability: p / 100.0,
        }
    }
}

fn recursion(
    problems: &[Problem],
    rem_money: usize,
    bits: usize,
    memo: &mut Array2<Option<f64>>,
) -> f64 {
    if let Some(exp) = memo[(rem_money, bits)] {
        return exp;
    }

    let max_exp = enumerate(problems)
        .filter_map(|(i, problem)| {
            if bits >> i & 1 == 1 || problem.cost > rem_money {
                return None;
            }

            let exp = (recursion(problems, rem_money - problem.cost, bits | 1 << i, memo)
                + problem.score as f64)
                * problem.probability
                + recursion(problems, rem_money - problem.cost, bits, memo)
                    * (1.0 - problem.probability);
            Some(exp)
        })
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap_or(0.0);

    memo[(rem_money, bits)] = Some(max_exp);

    max_exp
}

/// If the right-hand side is greater than the left-hand side,
/// clones the right-hand side, bind it to the left-hand side,
/// and return `true`.
/// If the right-hand side is less than or equal to the left-hand side,
/// does nothing and returns `false`.
///
/// The left-hand and right-hand sides must be the same type
/// and must implement the `Clone` trait.
///
/// # Examples
///
/// ```
/// let mut x = 5;
///
/// assert_eq!(chmax!(x, 3), false);
/// assert_eq!(x, 5);
///
/// assert_eq!(chmax!(x, 7), true);
/// assert_eq!(x, 7);
/// ```
#[macro_export]
macro_rules! chmax {
    ($lhs: expr, $rhs: expr) => {
        if $rhs > $lhs {
            let temp = $rhs.clone();
            $lhs = temp;
            true
        } else {
            false
        }
    };
}
