use std::{cmp::Reverse, collections::VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        pp: [Usize1; n - 1],
        mut xy: [(Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (i, &p) in pp.iter().enumerate() {
        graph[p].push(i + 1);
    }

    let mut dist = vec![n; n];
    dist[0] = 0;
    let mut queue = VecDeque::from(vec![0]);
    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            if dist[cur] + 1 < dist[next] {
                dist[next] = dist[cur] + 1;
                queue.push_back(next);
            }
        }
    }

    xy.sort_unstable_by_key(|&(x, y)| Reverse(dist[x] + y));

    let mut insurance = vec![false; n];

    for &(x, y) in &xy {
        rec(&graph, &mut insurance, x, y);
    }

    let ans = insurance.iter().filter(|&&x| x).count();
    println!("{}", ans);
}

fn rec(graph: &Vec<Vec<usize>>, insurance: &mut Vec<bool>, cur: usize, rem: usize) {
    if insurance[cur] {
        return;
    }

    insurance[cur] = true;

    if rem == 0 {
        return;
    }

    for &next in &graph[cur] {
        rec(graph, insurance, next, rem - 1);
    }
}
