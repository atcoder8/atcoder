use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable();
    aa.dedup();

    println!("{}\n{}", aa.len(), aa.iter().join(" "));
}
