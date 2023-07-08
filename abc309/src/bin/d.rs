use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n1, n2, m): (usize, usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let ab1 = ab.iter().filter(|x| x.0 < n1).cloned().collect_vec();
    let ab2 = ab
        .iter()
        .filter(|x| x.0 >= n1)
        .map(|&(a, b)| (a - n1, b - n1))
        .collect_vec();

    let ans = max_dist(n1, &ab1, 0) + max_dist(n2, &ab2, n2 - 1) + 1;
    println!("{}", ans);
}

fn max_dist(n: usize, edges: &Vec<(usize, usize)>, start: usize) -> usize {
    let mut graph = vec![vec![]; n];
    for &(a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut queue = VecDeque::from(vec![start]);
    let mut dists = vec![n; n];
    dists[start] = 0;
    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            if dists[cur] + 1 < dists[next] {
                dists[next] = dists[cur] + 1;
                queue.push_back(next);
            }
        }
    }

    *dists.iter().max().unwrap()
}
