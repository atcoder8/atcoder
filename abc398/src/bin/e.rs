use std::{collections::BTreeSet, io::Write};

use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use proconio::{input_interactive, marker::Usize1};

fn main() {
    input_interactive!(n: usize, uv: [(Usize1, Usize1); n - 1]);

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let dist_matrix = calc_distances(n, &uv);
    let mut odd_pairs: BTreeSet<(usize, usize)> = (0..n)
        .tuple_combinations()
        .filter(|&(i, j)| {
            let dist = dist_matrix[(i, j)];
            dist >= 3 && dist % 2 == 1
        })
        .collect();

    let mut my_turn = odd_pairs.len() % 2 == 1;

    println!("{}", if my_turn { "First" } else { "Second" });
    std::io::stdout().flush().unwrap();

    loop {
        if my_turn {
            let (i, j) = odd_pairs.pop_first().unwrap();
            println!("{} {}", i + 1, j + 1);
        } else {
            input_interactive!((i, j): (i64, i64));

            if i == -1 && j == -1 {
                break;
            }

            let mut i = i as usize - 1;
            let mut j = j as usize - 1;
            if i > j {
                std::mem::swap(&mut i, &mut j);
            }
            odd_pairs.remove(&(i, j));
        }

        my_turn = !my_turn;
    }
}

fn calc_distances(num_vertices: usize, edges: &[(usize, usize)]) -> Array2<usize> {
    let mut dist_matrix = Array2::from_elem((num_vertices, num_vertices), None::<usize>);
    for i in 0..num_vertices {
        dist_matrix[(i, i)] = Some(0);
    }
    for &(u, v) in edges {
        dist_matrix[(u, v)] = Some(1);
        dist_matrix[(v, u)] = Some(1);
    }

    for (mid, from, to) in iproduct!(0..num_vertices, 0..num_vertices, 0..num_vertices) {
        if let (Some(dist1), Some(dist2)) = (dist_matrix[(from, mid)], dist_matrix[(mid, to)]) {
            chmin_for_option(&mut dist_matrix[(from, to)], dist1 + dist2);
        }
    }

    dist_matrix.map(|dist| dist.unwrap())
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
