use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let s = if n == 1 {
        vec![1]
    } else {
        (2..=n).step_by(2).collect()
    };
    println!("{}\n{}", s.len(), s.iter().join(" "));
}
