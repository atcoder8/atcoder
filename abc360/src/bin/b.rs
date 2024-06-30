use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (s, t): (Chars, Chars),
    }

    for c in 1..s.len() {
        for w in c..s.len() {
            let mut u = vec![];
            for chunk in s.chunks(w) {
                if chunk.len() >= c {
                    u.push(chunk[c - 1]);
                }
            }

            if u == t {
                return true;
            }
        }
    }

    false
}
