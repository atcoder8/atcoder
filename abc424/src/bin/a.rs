use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (a, b, c): (u8, u8, u8),
    }

    let ans = ![a, b, c].iter().all_unique();
    println!("{}", if ans { "Yes" } else { "No" });
}
