use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
    }

    let mut groups = vec![vec![]; 100];
    for (i, &p) in enumerate(&pp) {
        groups[p].push(i);
    }

    let mut ranks = vec![0_usize; n];
    let mut rank = 1;
    for group in groups.iter().rev() {
        group.iter().for_each(|&i| ranks[i] = rank);
        rank += group.len();
    }

    println!("{}", ranks.iter().join("\n"));
}
