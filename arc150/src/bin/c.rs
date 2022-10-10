use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m, k): (usize, usize, usize),
        uv: [(Usize1, Usize1); m],
        aa: [Usize1; n],
        bb: [Usize1; k],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &uv {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let init_b_idx = if aa[0] == bb[0] { 1 } else { 0 };

    let mut deq = VecDeque::from(vec![(0, init_b_idx)]);
    let mut visited = vec![false; n];

    loop {
        let (cur_node, cur_b_progress) = deq.pop_front().unwrap();

        if visited[cur_node] {
            continue;
        }

        visited[cur_node] = true;

        if cur_node == n - 1 {
            return cur_b_progress == k;
        }

        for &next_node in &graph[cur_node] {
            let progress_flag = cur_b_progress < k && aa[next_node] == bb[cur_b_progress];

            if progress_flag {
                deq.push_back((next_node, cur_b_progress + 1));
            } else {
                deq.push_front((next_node, cur_b_progress));
            }
        }
    }
}
