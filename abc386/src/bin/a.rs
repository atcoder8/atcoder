use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        cards: [usize; 4],
    }

    let counts = cards
        .iter()
        .sorted_unstable()
        .dedup_with_count()
        .map(|v| v.0)
        .sorted_unstable()
        .collect_vec();
    let ans = counts.len() == 2 && (counts[0] == 1 || counts[1] == 2);
    println!("{}", if ans { "Yes" } else { "No" });
}
