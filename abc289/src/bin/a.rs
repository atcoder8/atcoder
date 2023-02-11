use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    for &c in &s {
        print!("{}", if c == '1' { 0 } else { 1 });
    }
    println!();
}
