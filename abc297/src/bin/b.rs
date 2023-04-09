use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let b1 = s.iter().find_position(|&&c| c == 'B').unwrap().0;
    let b2 = b1
        + 1
        + s[(b1 + 1)..]
            .iter()
            .find_position(|&&c| c == 'B')
            .unwrap()
            .0;

    let r1 = s.iter().find_position(|&&c| c == 'R').unwrap().0;
    let r2 = r1
        + 1
        + s[(r1 + 1)..]
            .iter()
            .find_position(|&&c| c == 'R')
            .unwrap()
            .0;

    let k = s.iter().find_position(|&&c| c == 'K').unwrap().0;

    let ans = (b1 % 2 != b2 % 2) && r1 < k && k < r2;
    println!("{}", if ans { "Yes" } else { "No" });
}
