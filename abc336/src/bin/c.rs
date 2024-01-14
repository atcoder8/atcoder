use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut digits = vec![];
    let mut t = n - 1;
    while t != 0 {
        digits.push(t % 5 * 2);
        t /= 5;
    }

    if digits.is_empty() {
        digits = vec![0];
    }

    println!("{}", digits.iter().rev().join(""));
}
