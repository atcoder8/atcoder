use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    s.remove(s.len() / 2);

    let ans = s.iter().join("");
    println!("{ans}");
}
