use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> usize {
    input! {
        n: usize,
        s: String,
    }

    let num_zeros = s.chars().filter(|&ch| ch == '0').count();
    let counts = [num_zeros, n - num_zeros];
    s.chars()
        .dedup_with_count()
        .map(|(cnt, dest)| {
            let num_dest_chars = counts[dest.to_digit(10).unwrap() as usize];
            2 * (num_dest_chars - cnt) + n - num_dest_chars
        })
        .min()
        .unwrap()
}
