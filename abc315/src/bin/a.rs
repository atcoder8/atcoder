use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let exclude = ['a', 'e', 'i', 'o', 'u'];
    for &c in &s {
        if !exclude.contains(&c) {
            print!("{c}");
        }
    }
}
