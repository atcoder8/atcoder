use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let seq = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let ans = 1
        + seq
            .iter()
            .tuple_windows()
            .map(|(&v1, &v2)| (10 + v1 - v2) % 10 + 1)
            .sum::<usize>()
        + seq.last().unwrap();
    println!("{}", ans);
}
