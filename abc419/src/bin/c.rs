use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        rc: [(usize, usize); n],
    }

    let (min_r, max_r) = rc.iter().map(|&(r, _)| r).minmax().into_option().unwrap();
    let (min_c, max_c) = rc.iter().map(|&(_, c)| c).minmax().into_option().unwrap();
    let min_time = ((max_r - min_r + 1) / 2).max((max_c - min_c + 1) / 2);
    println!("{}", min_time);
}
