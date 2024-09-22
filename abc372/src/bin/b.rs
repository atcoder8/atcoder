use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        m: usize,
    }

    let mut aa = vec![];
    let mut t = m;
    for exp in 0..=10 {
        aa.extend(vec![exp; t % 3]);
        t /= 3;
    }

    println!("{}\n{}", aa.len(), aa.iter().join(" "));
}
