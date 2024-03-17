use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut counts = vec![0; 26];
    for c in s.chars() {
        counts[c2u(c)] += 1;
    }

    let mut ans = 0;
    for (i, c) in enumerate(s.chars()) {
        counts[c2u(c)] -= 1;
        ans += s.len() - (i + 1) - counts[c2u(c)];
    }
    ans += !s.chars().all_unique() as usize;

    println!("{}", ans);
}

fn c2u(c: char) -> usize {
    (c as u8 - b'a') as usize
}
