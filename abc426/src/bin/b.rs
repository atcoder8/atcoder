use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    s.sort_unstable();

    let ans = if s[0] == s[1] { s[s.len() - 1] } else { s[0] };
    println!("{ans}");
}
