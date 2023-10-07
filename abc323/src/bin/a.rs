use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = s.iter().skip(1).step_by(2).all(|&c| c == '0');
    println!("{}", if ans { "Yes" } else { "No" });
}
