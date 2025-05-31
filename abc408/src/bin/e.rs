use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uvw: [(Usize1, Usize1, u32); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &uvw {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    // 立っているビットが`mask`に含まれる辺のみを用いて頂点Nまで移動できるか
    let is_ok = |mask: u32| {
        let mut graph = vec![vec![]; n];
        for &(u, v, weight) in &uvw {
            if weight | mask == mask {
                graph[u].push(v);
                graph[v].push(u);
            }
        }

        let mut visited = vec![false; n];
        let mut stack = vec![0];
        while let Some(curr) = stack.pop() {
            if visited[curr] {
                continue;
            }

            visited[curr] = true;

            stack.extend(graph[curr].iter().cloned());
        }

        visited[n - 1]
    };

    let mut mask = 2_u32.pow(30) - 1;
    for exp in (0_u32..30).rev() {
        if is_ok(mask ^ 2_u32.pow(exp)) {
            mask ^= 2_u32.pow(exp);
        }
    }

    println!("{}", mask);
}
