use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        sss: [[Usize1]; n],
    }

    let mut belong = vec![vec![]; m];
    for (i, ss) in sss.iter().enumerate() {
        for &s in ss {
            belong[s].push(i);
        }
    }

    let mut queue: VecDeque<(usize, usize)> = belong[0].iter().map(|&node| (node, 0)).collect();
    let mut visited_node = vec![false; n];
    let mut visited_value = vec![false; m];

    while let Some((node, dist)) = queue.pop_front() {
        for &shared_value in &sss[node] {
            if shared_value == m - 1 {
                return Some(dist);
            }

            if visited_value[shared_value] {
                continue;
            }

            visited_value[shared_value] = true;

            for &next_node in &belong[shared_value] {
                if visited_node[next_node] {
                    continue;
                }

                visited_node[next_node] = true;
                queue.push_back((next_node, dist + 1));
            }
        }
    }

    None
}
