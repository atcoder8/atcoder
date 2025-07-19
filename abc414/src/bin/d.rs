use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        xx: [usize; n],
    }

    let ans = xx
        .iter()
        .sorted_unstable()
        .tuple_windows()
        .map(|(x1, x2)| x2 - x1)
        .sorted_unstable()
        .take(n - m)
        .sum::<usize>();
    println!("{}", ans);
}
