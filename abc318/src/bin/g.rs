// unfinished

use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        (a, b, c): (Usize1, Usize1, Usize1),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    {
        // A -> B (without C)
        let mut queue = VecDeque::from(vec![a]);
        let mut visited = vec![false; n];
        visited[a] = true;
        while let Some(cur) = queue.pop_front() {
            for &next in &graph[cur] {
                if !visited[next] && next != c {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
        if !visited[b] {
            return false;
        }
    }

    {
        // B -> C (without A)
        let mut queue = VecDeque::from(vec![b]);
        let mut visited = vec![false; n];
        visited[b] = true;
        while let Some(cur) = queue.pop_front() {
            for &next in &graph[cur] {
                if !visited[next] && next != a {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
        if !visited[c] {
            return false;
        }
    }

    let mut queue = VecDeque::from(vec![a]);
    let mut visited = vec![false; n];
    visited[a] = true;
    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    visited[c]
}
