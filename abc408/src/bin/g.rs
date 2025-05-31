use itertools::Itertools;
use num::Integer;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve()).join("\n"));
}

fn solve() -> u128 {
    input! {
        (a, b, c, d): (u128, u128, u128, u128),
    }

    if a / b + 1 < (c + d - 1) / d - 1 {
        return 1;
    }

    let is_ok = |denom: u128| a * denom / b + 1 < (c * denom + d - 1) / d;

    let sum_numer = a + c;
    let sum_denom = b + d;
    (1..=sum_denom / sum_numer.gcd(&sum_denom))
        .find(|&denom| is_ok(denom))
        .unwrap()
}
