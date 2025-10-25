use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
        s: Chars,
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut top_nodes: Vec<Vec<Node>> = vec![vec![]; n];
    let mut queue = VecDeque::from_iter((0..n).filter_map(|start| {
        if s[start] == 'S' {
            Some((start, Node::new(start, 0)))
        } else {
            None
        }
    }));
    while let Some((curr, node)) = queue.pop_front() {
        let nodes = &mut top_nodes[curr];
        if insert_node(nodes, node) {
            queue.extend(
                graph[curr]
                    .iter()
                    .map(|&adj| (adj, Node::new(node.id, node.dist + 1))),
            );
        }
    }

    let ans = (0..n)
        .filter(|&i| s[i] == 'D')
        .map(|i| top_nodes[i][0].dist + top_nodes[i][1].dist)
        .join("\n");
    println!("{ans}");
}

#[derive(Debug, Clone, Copy)]
struct Node {
    id: usize,
    dist: usize,
}

impl Node {
    fn new(id: usize, dist: usize) -> Self {
        Self { id, dist }
    }
}

fn insert_node(nodes: &mut Vec<Node>, insert_node: Node) -> bool {
    if nodes.iter().any(|node| node.id == insert_node.id) {
        return false;
    }

    let insert_position = nodes.partition_point(|node| node.dist <= insert_node.dist);
    if insert_position >= 2 {
        return false;
    }

    nodes.insert(insert_position, insert_node);
    nodes.truncate(2);

    true
}
