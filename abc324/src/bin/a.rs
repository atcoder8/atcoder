use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    println!("{}", if aa.iter().all_equal() { "Yes" } else { "No" });
}
