use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let n = s.len();
    let ans = (0..=n)
        .tuple_combinations()
        .map(|(left, right)| s[left..right].to_string())
        .unique()
        .count();
    println!("{}", ans);
}
