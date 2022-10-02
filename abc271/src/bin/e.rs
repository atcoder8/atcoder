use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        abc: [(Usize1, Usize1, usize); m],
        ee: [Usize1; k],
    }

    let mut costs: Vec<Option<usize>> = vec![None; n];
    costs[0] = Some(0);

    for &e in &ee {
        let (from, to, edge_cost) = abc[e];

        if let Some(from_cost) = costs[from] {
            if costs[to].is_none() || from_cost + edge_cost < costs[to].unwrap() {
                costs[to] = Some(from_cost + edge_cost);
            }
        }
    }

    if let Some(ans) = costs[n - 1] {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
