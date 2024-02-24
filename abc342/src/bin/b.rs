use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }

    let mut p_to_idx = vec![0; n];
    for (i, &p) in enumerate(&pp) {
        p_to_idx[p] = i;
    }

    let solve = |(a, b): (usize, usize)| {
        if p_to_idx[a] < p_to_idx[b] {
            a + 1
        } else {
            b + 1
        }
    };

    let ans = ab.into_iter().map(solve).join("\n");
    println!("{}", ans);
}
