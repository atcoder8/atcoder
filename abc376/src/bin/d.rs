use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
    }

    let find_distance = |start: usize, labels: &mut [Option<usize>]| {
        let mut queue = VecDeque::from([(start, 1)]);
        while let Some((cur, dist)) = queue.pop_front() {
            if cur == 0 {
                return Some(dist);
            }

            if labels[cur] == Some(start) {
                continue;
            }

            labels[cur] = Some(start);

            queue.extend(graph[cur].iter().map(|&adj| (adj, dist + 1)));
        }

        None
    };

    let mut labels = vec![None; n];
    graph[0]
        .iter()
        .filter_map(|&start| find_distance(start, &mut labels))
        .min()
}
