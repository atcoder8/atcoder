use proconio::{input, marker::Usize1};

fn main() {
    let ans = solve();
    println!("{}", itertools::join(ans.iter().map(|&e| e + 1), " "));
}

fn solve() -> Vec<usize> {
    input! {
        (n, x, y): (usize, Usize1, Usize1),
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &uv {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut path = vec![];

    let mut stack = vec![(None, x, false)];

    while let Some((par, cur, visited)) = stack.pop() {
        if visited {
            path.pop();
            continue;
        }

        path.push(cur);
        stack.push((par, cur, true));

        if cur == y {
            return path;
        }

        for &next in graph[cur].iter() {
            if Some(next) != par {
                stack.push((Some(cur), next, false));
            }
        }
    }

    panic!()
}
