use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        ab: [(Usize1, Usize1); n - 1],
        vv: [Usize1; k],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let calc_dists = |start: usize| {
        let mut dists = vec![n; n];
        let mut stack = vec![(start, 0_usize)];
        while let Some((cur, dist)) = stack.pop() {
            if dists[cur] != n {
                continue;
            }

            dists[cur] = dist;

            stack.extend(graph[cur].iter().map(|&adj| (adj, dist + 1)));
        }

        dists
    };

    let dists_from_v0 = calc_dists(vv[0]);
    let farthest_node = *vv.iter().min_by_key(|&&v| dists_from_v0[v]).unwrap();
    let dists = calc_dists(farthest_node);

    let mut visited = vec![false; n];
    visited[farthest_node] = true;
    for &v in &vv {
        let mut cur = v;
        while !visited[cur] {
            visited[cur] = true;
            cur = *graph[cur]
                .iter()
                .find(|&&adj| dists[adj] + 1 == dists[cur])
                .unwrap();
        }
    }

    let ans = visited.iter().filter(|&&x| x).count();
    println!("{}", ans);
}
