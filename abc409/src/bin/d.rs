use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve()).join("\n"));
}

fn solve() -> String {
    input! {
        n: usize,
        mut s: Chars,
    }

    let Some(dec_pos) = (0..n - 1).find(|&i| s[i] > s[i + 1]) else {
        return s.iter().collect();
    };
    let larger_pos = s[dec_pos + 2..]
        .iter()
        .position(|&ch| ch > s[dec_pos])
        .map(|i| dec_pos + 2 + i)
        .unwrap_or(n);
    s.insert(larger_pos, s[dec_pos]);
    s.remove(dec_pos);

    s.iter().collect()
}
