use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut positions_each_a = vec![vec![0]; n];
    for (i, &a) in enumerate(&aa) {
        positions_each_a[a].push(i + 1);
    }
    positions_each_a
        .iter_mut()
        .for_each(|positions| positions.push(n + 1));

    let mut sum_dup = 0_usize;
    for positions in &positions_each_a {
        for (&pos1, &pos2) in positions.iter().tuple_windows() {
            sum_dup += calc_triangle_number(pos2 - pos1 - 1);
        }
    }

    let ans = n.pow(2) * (n + 1) / 2 - sum_dup;
    println!("{}", ans);
}

fn calc_triangle_number(n: usize) -> usize {
    n * (n + 1) / 2
}
