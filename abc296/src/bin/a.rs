use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let ans = (0..(n - 1)).all(|i| s[i] != s[i + 1]);
    println!("{}", if ans { "Yes" } else { "No" });
}
