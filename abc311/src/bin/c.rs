use itertools::join;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut visited = vec![false; n];
    let mut cur = 0;
    while !visited[cur] {
        visited[cur] = true;
        cur = aa[cur];
    }

    let mut t = cur;
    let mut ans = vec![];
    while {
        ans.push(t + 1);
        t = aa[t];

        t != cur
    } {}

    println!("{}\n{}", ans.len(), join(&ans, " "));
}
