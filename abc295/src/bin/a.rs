use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ww: [String; n],
    }

    let candidate = vec!["and", "not", "that", "the", "you"];
    let candidate = candidate.into_iter().map(|w| w.to_string()).collect_vec();

    let ans = ww.iter().any(|w| candidate.contains(w));
    println!("{}", if ans { "Yes" } else { "No" });
}
