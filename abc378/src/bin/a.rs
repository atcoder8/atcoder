use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        aa: [usize; 4],
    }

    let ans = aa
        .iter()
        .sorted_unstable()
        .dedup_with_count()
        .map(|(cnt, _)| cnt / 2)
        .sum::<usize>();
    println!("{}", ans);
}
