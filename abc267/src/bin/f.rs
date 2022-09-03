// unfinished

use std::collections::VecDeque;

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let mut ab = Vec::new();
    for _ in 0..n {
        ab.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        });
    }
    let q = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let mut uk = Vec::new();
    for _ in 0..q {
        uk.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        });
    }

    let mut graph = vec![vec![]; n as usize];
    for &(a, b) in ab.iter() {
        graph[a as usize].push(b);
        graph[b as usize].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut parent: Vec<Option<usize>> = vec![None; n];
    let mut dist = vec![0; n];
    let mut que = VecDeque::from(vec![0]);
    let mut visited = vec![false; n];
    visited[0] = true;

    while let Some(curr) = que.pop_front() {
        for &next in graph[curr].iter() {
            if visited[next] {
                continue;
            }

            parent[next] = Some(curr);
            dist[next] = dist[curr] + 1;
            que.push_back(next);
        }
    }

    let mut lca = vec![parent];
    for i in (0..).take_while(|&i| (n >> i) != 0) {
        for j in 0..n {
            lca[i + 1][j] = if let Some(p) = lca[i][j] {
                lca[i][p]
            } else {
                None
            }
        }
    }

    for &(u, k) in uk.iter() {
        
    }
}
