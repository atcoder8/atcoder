use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = (0..=s.len())
        .tuple_combinations()
        .map(|(left, right)| &s[left..right])
        .unique()
        .count();
    println!("{}", ans);
}
