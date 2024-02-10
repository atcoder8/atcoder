use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        abx: [(usize, usize, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for (i, &(a, b, x)) in enumerate(&abx) {
        graph[i].push((i + 1, a));
        graph[i].push((x, b));
    }

    let mut costs = vec![None; n];
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(cand_cost), cur)) = heap.pop() {
        if costs[cur].is_some() {
            continue;
        }

        costs[cur] = Some(cand_cost);

        heap.extend(
            graph[cur]
                .iter()
                .map(|&(next, weight)| (Reverse(cand_cost + weight), next)),
        );
    }

    println!("{}", costs[n - 1].unwrap());
}
