use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use itertools::Itertools;

fn main() {
    input! {
        (n, m): (usize, usize),
        xx: [usize; n],
        yy: [usize; n],
        mut abz: [(Usize1, Usize1, usize); m],
    }

    let kruskal = |edges: &mut Vec<(usize, usize, usize)>| {
        edges.sort_unstable_by_key(|edge| edge.2);

        let mut total_cost = 0;
        let mut uf = UnionFind::new(n + 2);

        for &mut (x, y, cost) in edges {
            if !uf.equiv(x, y) {
                total_cost += cost;
                uf.union(x, y);
            }
        }

        if uf.into_labeling()[..n].iter().all_equal() {
            Some(total_cost)
        } else {
            None
        }
    };

    let ans = (0..4)
        .filter_map(|bit| {
            let mut edges = vec![];

            if bit & 1 != 0 {
                for (i, &x) in xx.iter().enumerate() {
                    edges.push((i, n, x));
                }
            }

            if bit & 2 != 0 {
                for (i, &y) in yy.iter().enumerate() {
                    edges.push((i, n + 1, y));
                }
            }

            for &(a, b, z) in &abz {
                edges.push((a, b, z));
            }

            kruskal(&mut edges)
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
