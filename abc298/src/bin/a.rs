use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let ans = s.iter().any(|&c| c == 'o') && s.iter().all(|&c| c != 'x');
    println!("{}", if ans { "Yes" } else { "No" });
}
