use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut xx: [i64; n],
        uvw: [(Usize1, Usize1, u64); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &uvw {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let sum_cost = recursion(&graph, None, 0, &mut xx);
    println!("{}", sum_cost);
}

fn recursion(
    graph: &[Vec<(usize, u64)>],
    parent: Option<usize>,
    current: usize,
    potentials: &mut [i64],
) -> u64 {
    graph[current]
        .iter()
        .filter_map(|&(adjacent, weight)| {
            if Some(adjacent) == parent {
                return None;
            }

            let mut cost = recursion(graph, Some(current), adjacent, potentials);
            cost += potentials[adjacent].abs() as u64 * weight;
            potentials[current] += potentials[adjacent];
            potentials[adjacent] = 0;

            Some(cost)
        })
        .sum()
}
