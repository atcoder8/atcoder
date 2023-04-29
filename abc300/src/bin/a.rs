use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        cc: [usize; n],
    }

    let ans = cc.iter().find_position(|&&c| c == a + b).unwrap().0 + 1;
    println!("{}", ans);
}
