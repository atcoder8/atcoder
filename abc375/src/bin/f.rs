use itertools::{enumerate, iproduct, Itertools};
use ndarray::Array2;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, q): (usize, usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }
    let queries = (0..q)
        .map(|_| {
            input! {
                qt: usize,
            }

            if qt == 1 {
                input! {
                    i: Usize1,
                }

                Query::Close(i)
            } else {
                input! {
                    (x, y): (Usize1, Usize1),
                }

                Query::Output(x, y)
            }
        })
        .collect_vec();

    let mut passable = vec![true; m];
    for &query in &queries {
        if let Query::Close(i) = query {
            passable[i] = false;
        }
    }

    let mut dist_matrix = Array2::<Option<usize>>::from_elem((n, n), None);
    for i in 0..n {
        dist_matrix[(i, i)] = Some(0);
    }
    for (i, &(a, b, c)) in enumerate(&abc) {
        if passable[i] {
            dist_matrix[(a, b)] = Some(c);
            dist_matrix[(b, a)] = Some(c);
        }
    }
    for (mid, from, to) in iproduct!(0..n, 0..n, 0..n) {
        if let (Some(dist1), Some(dist2)) = (dist_matrix[(from, mid)], dist_matrix[(mid, to)]) {
            chmin_for_option(&mut dist_matrix[(from, to)], dist1 + dist2);
        }
    }

    let mut dists = vec![];
    for &query in queries.iter().rev() {
        match query {
            Query::Close(edge_idx) => {
                let (a, b, c) = abc[edge_idx];

                chmin_for_option(&mut dist_matrix[(a, b)], c);
                chmin_for_option(&mut dist_matrix[(b, a)], c);
                for (from, to) in iproduct!(0..n, 0..n) {
                    if let (Some(dist1), Some(dist2)) =
                        (dist_matrix[(from, a)], dist_matrix[(a, to)])
                    {
                        chmin_for_option(&mut dist_matrix[(from, to)], dist1 + dist2);
                    }
                }
                for (from, to) in iproduct!(0..n, 0..n) {
                    if let (Some(dist1), Some(dist2)) =
                        (dist_matrix[(from, b)], dist_matrix[(b, to)])
                    {
                        chmin_for_option(&mut dist_matrix[(from, to)], dist1 + dist2);
                    }
                }
            }
            Query::Output(x, y) => {
                dists.push(dist_matrix[(x, y)]);
            }
        }
    }

    let ans = dists
        .iter()
        .rev()
        .map(|&dist| match dist {
            Some(dist) => dist.to_string(),
            None => "-1".to_string(),
        })
        .join("\n");
    println!("{}", ans);
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Close(usize),
    Output(usize, usize),
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
