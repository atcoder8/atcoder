use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: String,
        t: Chars,
    }

    let mut progress = 0;
    for c in s.chars() {
        if c.to_uppercase().next().unwrap() == t[progress] {
            progress += 1;

            if progress == 3 {
                break;
            }
        }
    }

    let ans = progress == 3 || progress == 2 && t[2] == 'X';
    println!("{}", if ans { "Yes" } else { "No" });
}
