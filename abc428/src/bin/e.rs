use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let calc_distances = |start: usize| {
        let mut stack = vec![(start, 0_usize)];
        let mut distances = vec![n; n];
        while let Some((curr, dist)) = stack.pop() {
            if distances[curr] < n {
                continue;
            }

            distances[curr] = dist;

            stack.extend(graph[curr].iter().map(|&adj| (adj, dist + 1)));
        }

        distances
    };

    let distances1 = calc_distances(0);
    let u = distances1.iter().position_max().unwrap();
    let distance2 = calc_distances(u);
    let v = distance2.iter().position_max().unwrap();
    let distance3 = calc_distances(v);

    let output = (0..n)
        .map(|node| if distance2[node] > distance3[node] || (distance2[node] == distance3[node] && u > v) {
            u
        } else {
            v
        } + 1).join("\n");
    println!("{output}");
}
