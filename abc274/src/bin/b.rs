use itertools::{join, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        ccc: [Chars; h],
    }

    let xx = (0..w)
        .map(|j| (0..h).filter(|&i| ccc[i][j] == '#').count())
        .collect_vec();

    println!("{}", join(xx, " "));
}
