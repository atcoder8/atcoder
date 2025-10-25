use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let output = (0..n)
        .map(|i| if i < m { "OK" } else { "Too Many Requests" })
        .join("\n");
    println!("{output}");
}
