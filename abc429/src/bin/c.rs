use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut counts = vec![0_usize; n];
    for &a in &aa {
        counts[a] += 1;
    }

    let ans = counts
        .iter()
        .map(|&cnt| cnt * cnt.saturating_sub(1) / 2 * (n - cnt))
        .sum::<usize>();
    println!("{}", ans);
}
