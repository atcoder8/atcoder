use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut max_score = 0_usize;
    for root in 0..n {
        let leaf_levels = graph[root]
            .iter()
            .map(|&adj_node| graph[adj_node].len() - 1)
            .sorted_unstable()
            .collect_vec();
        for x in 1..=leaf_levels.len() {
            let level = leaf_levels[leaf_levels.len() - x];
            if level != 0 {
                let score = (level + 1) * x + 1;
                max_score = max_score.max(score);
            }
        }
    }
    println!("{}", n - max_score);
}
