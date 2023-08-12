use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n1, n2, m): (usize, usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let n = n1 + n2;

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dists = vec![n; n];

    let mut d1 = 0;
    let mut queue = VecDeque::from(vec![0]);
    dists[0] = 0;
    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            let candidate_dist = dists[cur] + 1;
            if candidate_dist < dists[next] {
                dists[next] = candidate_dist;
                d1 = d1.max(candidate_dist);
                queue.push_back(next);
            }
        }
    }

    let mut d2 = 0;
    let mut queue = VecDeque::from(vec![n - 1]);
    dists[n - 1] = 0;
    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            let candidate_dist = dists[cur] + 1;
            if candidate_dist < dists[next] {
                dists[next] = candidate_dist;
                d2 = d2.max(candidate_dist);
                queue.push_back(next);
            }
        }
    }

    let d = d1 + d2 + 1;
    println!("{}", d);
}
