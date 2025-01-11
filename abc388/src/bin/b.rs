use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, d): (usize, usize),
        tl: [(usize, usize); n],
    }

    let solve = |k: usize| tl.iter().map(|&(t, l)| t * (l + k)).max().unwrap();
    println!("{}", (1..=d).map(solve).join("\n"));
}
