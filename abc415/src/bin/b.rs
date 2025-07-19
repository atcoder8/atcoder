use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = (0..s.len())
        .filter(|&i| s[i] == '#')
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.map(|i| i + 1).join(","))
        .join("\n");
    println!("{}", ans);
}
