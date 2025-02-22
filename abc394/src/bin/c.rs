use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    let n = s.len();
    for i in (0..n - 1).rev() {
        if s[i..i + 2] == ['W', 'A'] {
            s[i..i + 2].copy_from_slice(&['A', 'C']);
        }
    }

    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
