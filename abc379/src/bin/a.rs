use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let (a, b, c) = n.chars().collect_tuple().unwrap();
    println!("{}{}{} {}{}{}", b, c, a, c, a, b);
}
