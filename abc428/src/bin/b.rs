use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        s: String,
    }

    let counter = (0..=n - k)
        .map(|i| s[i..i + k].to_string())
        .sorted_unstable()
        .dedup_with_count()
        .collect_vec();
    let x = counter.iter().map(|v| v.0).max().unwrap();
    let output = counter
        .iter()
        .filter(|v| v.0 == x)
        .map(|v| &v.1)
        .sorted_unstable()
        .join(" ");
    println!("{x}\n{output}");
}
