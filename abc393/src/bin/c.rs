use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut cnt = 0_usize;
    let mut graph = vec![BTreeSet::new(); n];
    for &(u, v) in &uv {
        if u == v || graph[u].contains(&v) {
            cnt += 1;
        } else {
            graph[u].insert(v);
            graph[v].insert(u);
        }
    }

    println!("{}", cnt);
}
