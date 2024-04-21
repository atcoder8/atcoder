use itertools::{chain, Itertools};
use num_integer::Roots;
use proconio::input;

const MODULUS: usize = 998244353;

fn main() {
    input! {
        (n, q): (usize, usize),
        queries: [(usize, usize, usize); q],
    }

    let chunk_size = n.sqrt();

    let mut ans: Vec<usize> = vec![];
    let mut graph = vec![vec![]; n];

    for chunk in queries.chunks(chunk_size) {
        graph.iter_mut().for_each(|edges| edges.sort_unstable());

        let mut parents: Vec<Option<usize>> = vec![None; n];
        let mut visited = vec![false; n];

        for start in 0..n {
            let mut stack: Vec<(Option<usize>, usize)> = vec![(None, start)];
            while let Some((par, cur)) = stack.pop() {
                if visited[cur] {
                    continue;
                }

                visited[cur] = true;

                parents[cur] = par;

                stack.extend(graph[cur].iter().map(|&next| (Some(cur), next)));
            }
        }

        let mut add_edges = vec![];
        for &(ca, cb, cc) in chunk {
            let key = ans.last().cloned().unwrap_or(0);
            let qt = ca * (1 + key) % MODULUS % 2;
            let u = cb * (1 + key) % MODULUS % n;
            let v = cc * (1 + key) % MODULUS % n;

            if qt == 0 {
                add_edges.push((u, v));
                continue;
            }

            let find_hub = || {
                if graph[u].binary_search(&v).is_ok() {
                    return None;
                }

                for (u, v) in [(u, v), (v, u)] {
                    if let Some(par) = parents[u] {
                        if graph[par].binary_search(&v).is_ok() {
                            return Some(par);
                        }
                    }
                }

                let mut add_adj_u = vec![];
                let mut add_adj_v = vec![];
                for &edge in &add_edges {
                    if edge == (u, v) || edge == (v, u) {
                        return None;
                    }

                    for (a, b) in [(edge.0, edge.1), (edge.1, edge.0)] {
                        if a == u {
                            add_adj_u.push(b);
                        }

                        if a == v {
                            add_adj_v.push(b);
                        }
                    }
                }

                for (u, v) in [(u, v), (v, u)] {
                    if let Some(par) = parents[u] {
                        if graph[par].binary_search(&v).is_ok() {
                            return Some(par);
                        }
                    }
                }

                for (add_adj_u, v) in [(&add_adj_u, v), (&add_adj_v, u)] {
                    for &adj_u in add_adj_u {
                        if adj_u == v {
                            return None;
                        }

                        if graph[adj_u].binary_search(&v).is_ok() {
                            return Some(adj_u);
                        }
                    }
                }

                chain(add_adj_u, add_adj_v)
                    .tuple_windows()
                    .find_map(|(adj1, adj2)| if adj1 == adj2 { Some(adj1) } else { None })
            };

            ans.push(find_hub().map(|hub| hub + 1).unwrap_or(0));
        }

        for &(u, v) in &add_edges {
            graph[u].push(v);
            graph[v].push(u);
        }
    }

    println!("{}", ans.iter().join("\n"));
}
