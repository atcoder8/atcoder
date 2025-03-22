// unfinished

use itertools::izip;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut groups = vec![];
    let mut visited = vec![false; n];
    let mut belonging_group = vec![n; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut group = vec![];
        let mut stack = vec![(start, false)];
        while let Some((curr, parity)) = stack.pop() {
            if visited[curr] {
                continue;
            }

            visited[curr] = true;

            group.push((curr, parity));
            belonging_group[curr] = groups.len();

            stack.extend(graph[curr].iter().map(|&curr| (curr, !parity)));
        }

        groups.push(group);
    }

    let mut num_edges_by_group = vec![0_usize; groups.len()];
    for &(u, _) in &uv {
        num_edges_by_group[belonging_group[u]] += 1;
    }

    let grundy = izip!(groups, num_edges_by_group).fold(0_usize, |acc, (group, num_edges)| {
        let mut counts = [0_usize; 2];
        for &(_, parity) in &group {
            counts[parity as usize] += 1;
        }

        let mex = counts[0] * counts[0].saturating_sub(1) / 2
            + counts[1] * counts[1].saturating_sub(1) / 2
            - num_edges;
        acc ^ mex
    });
    println!("{}", if grundy == 0 { "Takahashi" } else { "Aoki" });
}
