use itertools::Itertools;
use proconio::input;

const WB: &str = "wbwbwwbwbwbw";

fn main() {
    input! {
        (w, b): (usize, usize),
    }

    let repeat = WB.repeat(100).chars().collect_vec();

    let sum = w + b;
    let ans =
        (0..WB.len()).any(|start| (start..start + sum).filter(|&i| repeat[i] == 'w').count() == w);
    println!("{}", if ans { "Yes" } else { "No" });
}
