// unfinished

use itertools::{izip, Itertools};
use proconio::input;
use scc::SCC;

fn main() {
    input! {
        _n: usize,
        s: String,
        t: String,
    }

    match solve(&s, &t) {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve(s: &str, t: &str) -> Option<usize> {
    if s == t {
        return Some(0);
    }

    if t.chars().sorted_unstable().dedup().count() == 26 {
        return None;
    }

    let s = s.chars().map(char_to_int).collect_vec();
    let t = t.chars().map(char_to_int).collect_vec();

    let mut graph = vec![vec![]; 26];
    let mut inv_graph = vec![vec![]; 26];
    let mut scc_graph = SCC::new(26);
    for (&from, &to) in izip!(&s, &t) {
        graph[from].push(to);
        inv_graph[to].push(from);
        scc_graph.add_edge(from, to);
    }
    graph.iter_mut().for_each(|edges| {
        edges.sort_unstable();
        edges.dedup();
    });
    inv_graph.iter_mut().for_each(|edges| {
        edges.sort_unstable();
        edges.dedup();
    });

    if graph.iter().any(|edges| edges.len() >= 2) {
        return None;
    }

    let scc = scc_graph.scc();
    let mut visited = vec![false; 26];
    let mut cnt = 0_usize;
    for &start in scc.iter().flatten() {
        if graph[start].is_empty() || visited[start] {
            continue;
        }

        let mut from = start;

        if from == graph[start][0] {
            continue;
        }

        let mut completed_cycle = true;
        loop {
            if visited[from] {
                completed_cycle &= from == start;
                break;
            }

            if graph[from].is_empty() {
                completed_cycle = false;
                break;
            }

            let to = graph[from][0];

            cnt += inv_graph[to].len();
            inv_graph[to].iter().for_each(|&from| visited[from] = true);
            completed_cycle &= inv_graph[to].len() == 1;

            from = to;
        }

        cnt += completed_cycle as usize;
    }

    Some(cnt)
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}

pub mod scc {
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
}
