use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    let ss: HashMap<String, usize> = st
        .iter()
        .enumerate()
        .map(|(i, (s, _))| (s.clone(), i))
        .collect();

    let mut visited = vec![false; n];

    for i in 0..n {
        if visited[i] {
            continue;
        }

        visited[i] = true;

        let mut que = VecDeque::from(vec![i]);

        while let Some(cur) = que.pop_front() {
            if let Some(&next_idx) = ss.get(&st[cur].1) {
                if next_idx == i {
                    return false;
                }

                visited[next_idx] = true;
                que.push_back(next_idx);
            } else {
                break;
            }
        }
    }

    true
}
