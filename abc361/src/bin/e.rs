use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, usize); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &abc {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let dists_from_0 = find_dists(&graph, 0);
    let u = dists_from_0.iter().position_max().unwrap();
    let dists_from_u = find_dists(&graph, u);
    let diameter = dists_from_u.iter().max().unwrap().unwrap();

    let sum_weight = abc.iter().map(|v| v.2).sum::<usize>();

    let ans = 2 * sum_weight - diameter;
    println!("{}", ans);
}

fn find_dists(graph: &[Vec<(usize, usize)>], start: usize) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut dists = vec![None::<usize>; n];
    let mut heap = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(cand_dist), cur)) = heap.pop() {
        if dists[cur].is_some_and(|dist| cand_dist >= dist) {
            continue;
        }

        dists[cur] = Some(cand_dist);

        heap.extend(
            graph[cur]
                .iter()
                .map(|&(next, weight)| (Reverse(cand_dist + weight), next)),
        );
    }

    dists
}
