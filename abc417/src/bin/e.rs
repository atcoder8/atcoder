use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        let path = solve();
        println!("{}", path.iter().map(|node| node + 1).join(" "));
    }
}

fn solve() -> Vec<usize> {
    input! {
        (n, m, x, y): (usize, usize, Usize1, Usize1),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut min_paths = vec![None::<Vec<usize>>; n];
    let mut heap = BinaryHeap::<(Reverse<Vec<usize>>, usize)>::from([(Reverse(vec![x]), x)]);
    while let Some((Reverse(path), curr)) = heap.pop() {
        if min_paths[curr]
            .as_ref()
            .is_some_and(|min_path| min_path <= &path)
        {
            continue;
        }

        min_paths[curr] = Some(path.clone());

        for &next in &graph[curr] {
            if path.contains(&next) {
                continue;
            }

            let mut next_path = path.clone();
            next_path.push(next);
            heap.push((Reverse(next_path), next));
        }
    }

    min_paths.remove(y).unwrap()
}
