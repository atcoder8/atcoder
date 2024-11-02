use proconio::{input, marker::Usize1};

// 共に次数が2である頂点対(u,v)のうちuからvの経路のうちu,vを除く頂点の次数が全て3であるものの個数を数える
// 次数2の頂点を始点とする

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

    let mut degrees = vec![0_usize; n];
    for &(u, v) in &uv {
        degrees[u] += 1;
        degrees[v] += 1;
    }

    let mut ans = 0_usize;
    let mut visited = vec![false; n];
    for start in 0..n {
        if degrees[start] != 2 || visited[start] {
            continue;
        }

        for &adj in graph[start].iter().filter(|&&adj| degrees[adj] == 3) {
            let mut count = 0_usize;
            let mut stack = vec![adj];
            while let Some(cur) = stack.pop() {
                if visited[cur] {
                    continue;
                }

                if degrees[cur] != 3 {
                    count += (degrees[cur] == 2) as usize;
                    continue;
                }

                visited[cur] = true;

                stack.extend(graph[cur].iter().cloned());
            }

            ans += count * count.saturating_sub(1) / 2;
        }
    }

    println!("{}", ans);
}
