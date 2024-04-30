use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        q: usize,
        uk: [(Usize1, usize); q],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let (farthest_node_1, _) = find_farthest_point(&graph, 0);
    let (farthest_node_2, _) = find_farthest_point(&graph, farthest_node_1);

    let mut queries_each_node = vec![vec![]; n];
    for (qi, &(u, k)) in enumerate(&uk) {
        queries_each_node[u].push((qi, k));
    }

    enum DFSNode {
        Forward { par: Option<usize>, cur: usize },
        Backward,
    }

    let mut other_nodes: Vec<Option<usize>> = vec![None; q];

    for farthest_node in [farthest_node_1, farthest_node_2] {
        let mut stack = vec![
            DFSNode::Backward,
            DFSNode::Forward {
                par: None,
                cur: farthest_node,
            },
        ];
        let mut path = vec![];

        while let Some(dfs_node) = stack.pop() {
            match dfs_node {
                DFSNode::Forward { par, cur } => {
                    for &(qi, k) in &queries_each_node[cur] {
                        if path.len() >= k {
                            other_nodes[qi] = Some(path[path.len() - k]);
                        }
                    }

                    path.push(cur);

                    stack.extend(
                        graph[cur]
                            .iter()
                            .filter(|&&next| Some(next) != par)
                            .flat_map(|&nxt| {
                                [
                                    DFSNode::Backward,
                                    DFSNode::Forward {
                                        par: Some(cur),
                                        cur: nxt,
                                    },
                                ]
                            }),
                    );
                }
                DFSNode::Backward => {
                    path.pop();
                }
            }
        }
    }

    for &other_node in &other_nodes {
        match other_node {
            Some(other_node) => println!("{}", other_node + 1),
            None => println!("-1"),
        }
    }
}

fn find_farthest_point(graph: &[Vec<usize>], start: usize) -> (usize, usize) {
    let n = graph.len();

    let mut dists = vec![None; n];
    let mut stack = vec![(start, 0)];

    while let Some((cur, dist)) = stack.pop() {
        if dists[cur].is_some() {
            continue;
        }

        dists[cur] = Some(dist);

        stack.extend(graph[cur].iter().map(|&next| (next, dist + 1)));
    }

    let farthest_node = dists.iter().position_max().unwrap();
    let farthest_dist = dists[farthest_node].unwrap();

    (farthest_node, farthest_dist)
}
