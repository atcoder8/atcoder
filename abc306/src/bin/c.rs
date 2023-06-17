use itertools::{join, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; 3 * n],
    }

    let mut indexes = vec![vec![]; n];
    for (i, &a) in aa.iter().enumerate() {
        indexes[a].push(i);
    }

    indexes.iter_mut().for_each(|x| x.sort_unstable());

    let mut seq = (0..n).collect_vec();
    seq.sort_unstable_by_key(|&x| indexes[x][1]);

    println!("{}", join(seq.iter().map(|&x| x + 1), " "));
}
