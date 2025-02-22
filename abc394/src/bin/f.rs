// unfinished

use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = None;
    let mut visited = vec![false; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }

        let num_nodes = rec(&graph, None, start, &mut visited);
        if num_nodes >= 5 {
            chmax_for_option(&mut ans, num_nodes);
        }
    }

    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn rec(graph: &[Vec<usize>], par: Option<usize>, cur: usize, visited: &mut [bool]) -> usize {
    visited[cur] = true;

    if par.is_some() && graph[cur].len() < 4 {
        return 1;
    }

    let num_next_nodes = if graph[cur].len() >= 4 { 4 } else { 1 } - par.is_some() as usize;
    let num_nodes = graph[cur]
        .iter()
        .filter(|&&adj| Some(adj) != par)
        .map(|&next| rec(graph, Some(cur), next, visited))
        .sorted_by_key(|&cnt| Reverse(cnt))
        .take(num_next_nodes)
        .sum::<usize>()
        + 1;
    num_nodes
}

/// If `value` is `None` or contains a value less than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmax_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost >= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
