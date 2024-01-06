use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [Usize1; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        if aa[v] >= aa[u] {
            graph[u].push(v);
        }

        if aa[u] >= aa[v] {
            graph[v].push(u);
        }
    }

    let mut scores = vec![0_usize; n];
    let mut heap = BinaryHeap::from([(Reverse(aa[0]), 1, 0)]);
    while let Some((_, cand_score, cur)) = heap.pop() {
        let score = &mut scores[cur];

        if *score >= cand_score {
            continue;
        }

        *score = cand_score;

        heap.extend(graph[cur].iter().map(|&next| {
            (
                Reverse(aa[next]),
                cand_score + (aa[next] > aa[cur]) as usize,
                next,
            )
        }));
    }

    println!("{}", scores[n - 1]);
}
