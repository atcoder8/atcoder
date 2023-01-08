use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &uv {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut visited = vec![false; n];

    let mut ans = 0;

    dfs(&graph, &mut visited, 0, &mut ans);

    println!("{}", ans.min(1_000_000));
}

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, cur: usize, count: &mut usize) {
    *count += 1;

    if *count >= 1_000_000 {
        return;
    }

    visited[cur] = true;

    for &next in &graph[cur] {
        if !visited[next] {
            dfs(graph, visited, next, count);
        }
    }

    visited[cur] = false;
}
