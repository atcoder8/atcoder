use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (s, t): (Chars, Chars),
    }

    if t.len() > s.len() {
        return false;
    }

    for i in 0..(s.len() - t.len() + 1) {
        if s[i..(i + t.len())] == t[..] {
            return true;
        }
    }

    false
}
