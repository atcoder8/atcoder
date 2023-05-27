use std::collections::VecDeque;

use itertools::join;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    let mut degrees = vec![0; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);

        degrees[u] += 1;
        degrees[v] += 1;
    }

    let start = (0..n).find(|&i| degrees[i] == 1).unwrap();

    let mut dists = vec![n; n];
    dists[start] = 0;
    let mut queue = VecDeque::from(vec![start]);
    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            if dists[cur] + 1 < dists[next] {
                dists[next] = dists[cur] + 1;
                queue.push_back(next);
            }
        }
    }

    let mut levels = vec![];
    for i in 0..n {
        if dists[i] % 3 == 1 {
            levels.push(degrees[i]);
        }
    }
    levels.sort_unstable();

    println!("{}", join(levels, " "));
}
