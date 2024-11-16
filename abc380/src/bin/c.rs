use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (_n, k): (usize, usize),
        s: String,
    }

    let mut counts = s.chars().dedup_with_count().collect_vec();
    if counts[0].1 == '1' {
        counts.insert(0, (0, '0'));
    }
    counts.swap(2 * k - 2, 2 * k - 1);

    let ans = counts
        .iter()
        .flat_map(|&(cnt, c)| std::iter::repeat(c).take(cnt))
        .collect::<String>();
    println!("{}", ans);
}
