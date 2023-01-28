use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    if m != n - 1 {
        return false;
    }

    let mut degrees = vec![0_usize; n];
    let mut uf: UnionFind<usize> = UnionFind::new(n);

    for &(u, v) in &uv {
        degrees[u] += 1;
        degrees[v] += 1;
        uf.union(u, v);
    }

    uf.into_labeling().iter().all_equal() && degrees.iter().all(|&degree| degree <= 2)
}
