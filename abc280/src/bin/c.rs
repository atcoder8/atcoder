use proconio::{input, marker::Chars};

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        s: Chars,
        t: Chars,
    }

    for i in 0..s.len() {
        if s[i] != t[i] {
            return i + 1;
        }
    }

    s.len() + 1
}
