use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut cnt = 0;
    rec(&graph, None, 0, &mut cnt);
    println!("{}", cnt);
}

fn rec(graph: &Vec<Vec<usize>>, par: Option<usize>, cur: usize, cnt: &mut usize) -> usize {
    let n = graph.len();

    let mut descendants = vec![];

    for &next in &graph[cur] {
        if Some(next) == par {
            continue;
        }

        descendants.push(rec(graph, Some(cur), next, cnt));
    }

    let child_sum: usize = descendants.iter().sum();
    let other_sum = n - 1 - child_sum;
    descendants.push(other_sum);

    let mut add = 0;
    add += n.saturating_sub(1) * n.saturating_sub(2) * n.saturating_sub(3) / 6;
    add -= descendants
        .iter()
        .map(|x| x * x.saturating_sub(1) * x.saturating_sub(2))
        .sum::<usize>()
        / 6;
    add -= descendants
        .iter()
        .map(|x| x * x.saturating_sub(1) * (n - 1 - x))
        .sum::<usize>()
        / 2;

    *cnt += add;

    descendants.pop();
    descendants.iter().sum::<usize>() + 1
}
