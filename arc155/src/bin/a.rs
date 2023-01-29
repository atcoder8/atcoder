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
        (n, k): (usize, usize),
        s: Chars,
    }

    let k = k % (2 * n);

    let mut t = vec!['\0'; k];

    for i in 0..k.min(n) {
        t[i] = s[n - 1 - i];
    }

    for i in 0..k.min(n) {
        t[k - 1 - i] = s[i];
    }

    is_ok(&s.iter().chain(&t).cloned().collect()) && is_ok(&t.iter().chain(&s).cloned().collect())
}

fn is_ok(s: &Vec<char>) -> bool {
    let n = s.len();

    (0..(n / 2)).all(|i| s[i] == s[n - 1 - i])
}
