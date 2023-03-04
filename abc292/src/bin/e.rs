use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &uv {
        graph[a].push(b);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let bfs = |node: usize| {
        let mut ret = 0;
        let mut queue = VecDeque::from(vec![node]);
        let mut visited = vec![false; n];
        visited[node] = true;

        while let Some(node) = queue.pop_front() {
            for &next_node in &graph[node] {
                if visited[next_node] {
                    continue;
                }

                visited[next_node] = true;
                queue.push_back(next_node);
                ret += 1;
            }
        }

        ret
    };

    let ans = (0..n).map(|node| bfs(node)).sum::<usize>() - uv.len();
    println!("{}", ans);
}
