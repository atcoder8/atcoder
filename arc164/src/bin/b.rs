use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
        cc: [usize; n],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut labels: Vec<Option<usize>> = vec![None; n];
    for start in 0..n {
        if labels[start].is_some() {
            continue;
        }

        labels[start] = Some(start);

        let mut queue = VecDeque::from(vec![start]);
        while let Some(cur) = queue.pop_front() {
            for &next in &graph[cur] {
                if let Some(next_label) = labels[next] {
                    if next_label == labels[cur].unwrap() && cc[next] == cc[cur] {
                        return true;
                    }
                } else {
                    if cc[cur] != cc[next] {
                        labels[next] = Some(start);
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    false
}
