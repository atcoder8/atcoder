use std::collections::BinaryHeap;

use itertools::{join, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        ab: [(Usize1, Usize1); m],
        ph: [(Usize1, usize); k],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut protected = vec![false; n];
    let mut rem = vec![0; n];
    let mut heap = BinaryHeap::new();
    for &(p, h) in &ph {
        protected[p] = true;
        rem[p] = h;
        heap.push((h, p));
    }

    while let Some((health, cur)) = heap.pop() {
        for &next in &graph[cur] {
            protected[next] = true;

            if health - 1 > rem[next] {
                rem[next] = health - 1;
                heap.push((health - 1, next));
            }
        }
    }

    let ans = (1..=n).filter(|&i| protected[i - 1]).collect_vec();
    println!("{}\n{}", ans.len(), join(ans, " "));
}
