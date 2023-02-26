use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = s.iter().find_position(|&&c| c.is_uppercase()).unwrap().0 + 1;
    println!("{}", ans);
}
