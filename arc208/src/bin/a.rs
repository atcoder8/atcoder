use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| if solve() { "Alice" } else { "Bob" })
        .join("\n");
    println!("{output}");
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let or = aa.iter().fold(0, |acc, a| acc | a);
    let xor = aa.iter().fold(0, |acc, a| acc ^ a);
    or != xor
}
