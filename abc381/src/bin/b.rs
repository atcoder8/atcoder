use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = s.len() % 2 == 0
        && s.chunks(2).all(|chunk| chunk[0] == chunk[1])
        && s.iter()
            .sorted_unstable()
            .dedup_with_count()
            .all(|(cnt, _)| cnt == 2);
    println!("{}", if ans { "Yes" } else { "No" });
}
