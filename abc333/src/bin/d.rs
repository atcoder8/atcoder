use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    println!("{}", rec(&graph, None, 0));
}

fn rec(graph: &[Vec<usize>], par: Option<usize>, cur: usize) -> usize {
    if graph[cur].len() == 1 {
        return 1;
    }

    let mut child_costs = vec![];
    for &next in &graph[cur] {
        if Some(next) == par {
            continue;
        }

        child_costs.push(rec(graph, Some(cur), next));
    }

    let child_cost_sum = child_costs.iter().sum::<usize>();

    if cur == 0 {
        child_cost_sum - child_costs.iter().max().unwrap() + 1
    } else {
        child_cost_sum + 1
    }
}
