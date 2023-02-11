use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        if let Some(ans) = solve() {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        cc: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &uv {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut queue = VecDeque::from(vec![(0, n - 1, 0)]);

    let mut visited = vec![vec![false; n]; n];
    visited[0][n - 1] = true;

    while let Some((cur_t_node, cur_a_node, cur_move_cnt)) = queue.pop_front() {
        if cur_t_node == n - 1 && cur_a_node == 0 {
            return Some(cur_move_cnt);
        }

        for &next_t_node in &graph[cur_t_node] {
            for &next_a_node in &graph[cur_a_node] {
                if cc[next_t_node] != cc[next_a_node] && !visited[next_t_node][next_a_node] {
                    visited[next_t_node][next_a_node] = true;
                    queue.push_back((next_t_node, next_a_node, cur_move_cnt + 1));
                }
            }
        }
    }

    None
}
