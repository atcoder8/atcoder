use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    s.sort_unstable();

    let ans = s == ['A', 'B', 'C'];
    println!("{}", if ans { "Yes" } else { "No" });
}
