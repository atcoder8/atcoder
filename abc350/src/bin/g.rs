// unfinished

use fixedbitset::FixedBitSet;
use itertools::Itertools;
use proconio::input;

const MODULUS: usize = 998244353;

fn main() {
    input! {
        (n, q): (usize, usize),
        queries: [(usize, usize, usize); q],
    }

    let mut ans: Vec<usize> = vec![];

    let mut graph = vec![FixedBitSet::with_capacity(n); n];
    for &(ca, cb, cc) in &queries {
        let key = ans.last().cloned().unwrap_or(0);
        let qt = ca * (1 + key) % MODULUS % 2;
        let u = cb * (1 + key) % MODULUS % n;
        let v = cc * (1 + key) % MODULUS % n;

        if qt == 0 {
            graph[u].insert(v);
            graph[v].insert(u);
        } else {
            let mut intersection = graph[u].clone();
            intersection.intersect_with(&graph[v]);

            let elem = intersection.ones().next().map(|elem| elem + 1).unwrap_or(0);
            ans.push(elem);
        }
    }

    println!("{}", ans.iter().join("\n"));
}
