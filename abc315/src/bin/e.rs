use std::collections::VecDeque;

use im_rc::HashSet;
use itertools::join;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ppp: [[Usize1]; n],
    }

    let mut graph = vec![vec![]; n];
    let mut scc_graph = SCC::new(n);
    for (i, pp) in ppp.iter().enumerate() {
        for &p in pp {
            graph[i].push(p);
            scc_graph.add_edge(p, i);
        }
    }
    let scc = scc_graph.scc();
    let mut inv = vec![0; n];
    for (i, g) in scc.iter().enumerate() {
        inv[g[0]] = i;
    }

    let mut set = HashSet::new();
    let mut queue = VecDeque::from(vec![0]);
    let mut visited = vec![false; n];
    visited[0] = true;
    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            if visited[next] {
                continue;
            }

            visited[next] = true;
            queue.push_back(next);
            set.insert(next);
        }
    }

    let mut ans = vec![];
    for v in &scc {
        let e = v[0];
        if e != 0 && visited[e] {
            ans.push(e + 1);
        }
    }
    println!("{}", join(ans, " "));
}

#[derive(Debug, Clone)]
pub struct SCC {
    graph: Vec<Vec<usize>>,
    inv_graph: Vec<Vec<usize>>,
}

impl SCC {
    pub fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            inv_graph: vec![vec![]; n],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
        self.inv_graph[to].push(from);
    }

    pub fn scc(&self) -> Vec<Vec<usize>> {
        let n = self.graph.len();

        let mut order = vec![];
        let mut visited = vec![false; n];
        for start_node in 0..n {
            if !visited[start_node] {
                order.append(&mut post_order_traversal(
                    &self.graph,
                    &mut visited,
                    start_node,
                ));
            }
        }

        let mut scc = vec![];
        let mut visited = vec![false; n];
        for &start_node in order.iter().rev() {
            if !visited[start_node] {
                scc.push(post_order_traversal(
                    &self.inv_graph,
                    &mut visited,
                    start_node,
                ));
            }
        }

        scc
    }
}

fn post_order_traversal(
    graph: &[Vec<usize>],
    visited: &mut [bool],
    start_node: usize,
) -> Vec<usize> {
    let mut post_order = vec![];

    let mut stack = vec![(start_node, false)];

    while let Some((node, back)) = stack.pop() {
        if back {
            post_order.push(node);
        }

        if visited[node] {
            continue;
        }

        visited[node] = true;

        stack.push((node, true));

        stack.extend(graph[node].iter().map(|&next_node| (next_node, false)));
    }

    post_order
}
