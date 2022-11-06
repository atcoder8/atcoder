use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let result = s.iter().rev().find_position(|&&c| c == 'a');

    if let Some((pos, _)) = result {
        println!("{}", s.len() - pos);
    } else {
        println!("-1");
    }
}
