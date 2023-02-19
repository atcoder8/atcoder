use proconio::{input, marker::Chars};

fn main() {
    input! {
        (_n, k): (usize, usize),
        s: Chars,
    }

    let mut rem = k;

    for &c in &s {
        if c == 'o' && rem != 0 {
            rem -= 1;
            print!("o");
        } else {
            print!("x");
        }
    }
    println!();
}
