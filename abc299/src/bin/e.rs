use std::collections::VecDeque;

use itertools::join;
use proconio::{input, marker::Usize1};

fn main() {
    if let Some(ans) = solve() {
        println!("Yes\n{}", join(ans.iter().map(|&x| x as usize), ""));
    } else {
        println!("No");
    }
}

fn solve() -> Option<Vec<bool>> {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
        k: usize,
        pd: [(Usize1, usize); k],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &uv {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut black = vec![true; n];

    for &(p, d) in &pd {
        let mut dist = vec![n; n];
        dist[p] = 0;
        let mut queue = VecDeque::from(vec![p]);

        while let Some(cur) = queue.pop_front() {
            for &next in &graph[cur] {
                if dist[cur] + 1 < dist[next] {
                    dist[next] = dist[cur] + 1;
                    queue.push_back(next);
                }
            }
        }

        (0..n)
            .filter(|&i| dist[i] < d)
            .for_each(|i| black[i] = false);
    }

    for &(p, d) in &pd {
        let mut dist = vec![n; n];
        dist[p] = 0;
        let mut queue = VecDeque::from(vec![p]);

        while let Some(cur) = queue.pop_front() {
            for &next in &graph[cur] {
                if dist[cur] + 1 < dist[next] {
                    dist[next] = dist[cur] + 1;
                    queue.push_back(next);
                }
            }
        }

        if (0..n).filter(|&i| dist[i] == d).all(|i| !black[i]) {
            return None;
        }
    }

    Some(black)
}
