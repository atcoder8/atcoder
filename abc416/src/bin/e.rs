use itertools::iproduct;
use ndarray::prelude::*;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
        (k, t): (usize, usize),
        dd: [Usize1; k],
        q: usize,
    }

    let queries = (0..q).map(|_| Query::read());

    let mut dist_array = Array2::from_elem((n + 1, n + 1), None::<usize>);
    for i in 0..n {
        dist_array[(i, i)] = Some(0);
    }
    for &(a, b, c) in &abc {
        chmin_for_option(&mut dist_array[(a, b)], c);
        chmin_for_option(&mut dist_array[(b, a)], c);
    }
    for &d in &dd {
        dist_array[(d, n)] = Some(t);
        dist_array[(n, d)] = Some(0);
    }
    for (mid, from, to) in iproduct!(0..n + 1, 0..n + 1, 0..n + 1) {
        if let (Some(dist1), Some(dist2)) = (dist_array[(from, mid)], dist_array[(mid, to)]) {
            chmin_for_option(&mut dist_array[(from, to)], dist1 + dist2);
        }
    }

    for query in queries {
        match query {
            Query::BuildRoad { x, y, t } => {
                chmin_for_option(&mut dist_array[(x, y)], t);
                chmin_for_option(&mut dist_array[(y, x)], t);

                for mid in [x, y] {
                    for (from, to) in iproduct!(0..n + 1, 0..n + 1) {
                        if let (Some(dist1), Some(dist2)) =
                            (dist_array[(from, mid)], dist_array[(mid, to)])
                        {
                            chmin_for_option(&mut dist_array[(from, to)], dist1 + dist2);
                        }
                    }
                }
            }
            Query::BuildAirport(x) => {
                chmin_for_option(&mut dist_array[(x, n)], t);
                chmin_for_option(&mut dist_array[(n, x)], 0);

                for mid in [x, n] {
                    for (from, to) in iproduct!(0..n + 1, 0..n + 1) {
                        if let (Some(dist1), Some(dist2)) =
                            (dist_array[(from, mid)], dist_array[(mid, to)])
                        {
                            chmin_for_option(&mut dist_array[(from, to)], dist1 + dist2);
                        }
                    }
                }
            }
            Query::OutputMinCost => {
                let min_cost = dist_array
                    .slice(s![0..n, 0..n])
                    .iter()
                    .filter_map(|&dist| dist)
                    .sum::<usize>();
                println!("{}", min_cost);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    BuildRoad { x: usize, y: usize, t: usize },
    BuildAirport(usize),
    OutputMinCost,
}

impl Query {
    fn read() -> Self {
        input! {
            qt: u8,
        }

        match qt {
            1 => {
                input! {
                    (x, y, t): (Usize1, Usize1, usize),
                }

                Query::BuildRoad { x, y, t }
            }
            2 => {
                input! {
                    x: Usize1,
                }

                Query::BuildAirport(x)
            }
            3 => Query::OutputMinCost,
            _ => panic!(),
        }
    }
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
