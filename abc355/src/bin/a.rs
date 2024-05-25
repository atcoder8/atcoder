use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let cand = (1..=3).filter(|&i| i != a && i != b).collect_vec();
    if cand.len() == 1 {
        println!("{}", cand[0]);
    } else {
        println!("-1");
    }
}
