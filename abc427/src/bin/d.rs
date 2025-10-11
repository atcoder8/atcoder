use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| if solve() { "Alice" } else { "Bob" })
        .join("\n");
    println!("{output}");
}

// Aliceが勝者になるかどうか調べる
fn solve() -> bool {
    input! {
        (n, m, k): (usize, usize, usize),
        s: Chars,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
    }

    let init: Vec<bool> = s.iter().map(|&ch| ch == 'A').collect();
    (0..2 * k).fold(init, |dp, _| {
        graph
            .iter()
            .map(|edges| edges.iter().any(|&next| !dp[next]))
            .collect()
    })[0]
}
