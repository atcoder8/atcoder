use itertools::izip;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        cc: [usize; n],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dists = vec![0; n];
    let mut sum_c_vec = vec![0; n];

    rec(&graph, None, 0, &cc, &mut dists, &mut sum_c_vec);

    let init_cost = izip!(&cc, &dists).map(|(c, dist)| c * dist).sum::<usize>();

    let mut cost_vec = vec![0; n];
    let mut stack: Vec<(Option<usize>, usize, usize)> = vec![(None, 0, init_cost)];
    while let Some((par, cur, cost)) = stack.pop() {
        cost_vec[cur] = cost;

        stack.extend(
            graph[cur]
                .iter()
                .filter(|&&next| Some(next) != par)
                .map(|&next| (Some(cur), next, cost + sum_c_vec[0] - 2 * sum_c_vec[next])),
        );
    }

    let ans = *cost_vec.iter().min().unwrap();
    println!("{}", ans);
}

fn rec(
    graph: &[Vec<usize>],
    par: Option<usize>,
    cur: usize,
    cc: &[usize],
    dists: &mut [usize],
    sum_c_vec: &mut [usize],
) {
    dists[cur] = par.map(|par| dists[par] + 1).unwrap_or(0);
    sum_c_vec[cur] = cc[cur];

    for &next in &graph[cur] {
        if Some(next) != par {
            rec(graph, Some(cur), next, cc, dists, sum_c_vec);
            sum_c_vec[cur] += sum_c_vec[next];
        }
    }
}
