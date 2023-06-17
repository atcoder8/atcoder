use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    for &c in &s {
        print!("{}", c);
        print!("{}", c);
    }
    println!();
}
