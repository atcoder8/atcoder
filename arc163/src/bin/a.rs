use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        println!("{}", if solve() { "Yes" } else { "No" });
    }
}

fn solve() -> bool {
    input! {
        n: usize,
        s: Chars,
    }

    for split_pos in 1..n {
        if s[..split_pos] < s[split_pos..] {
            return true;
        }
    }

    false
}
