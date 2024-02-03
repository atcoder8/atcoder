use std::str::FromStr;

use hashbag::HashBag;
use itertools::Itertools;
use num_bigint::BigUint;
use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [String; n],
    }

    let seq = aa
        .iter()
        .map(|a| BigUint::from_str(a).unwrap())
        .collect_vec();

    let mut counter = HashBag::new();
    for val in seq.clone() {
        counter.insert(val);
    }

    let mut ans = 0_usize;
    for val in &seq {
        ans += seq
            .iter()
            .filter(|x| x.is_multiple_of(val))
            .map(|x| counter.contains(&(x / val)))
            .sum::<usize>();
    }

    println!("{}", ans);
}
