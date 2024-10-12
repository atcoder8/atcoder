use itertools::enumerate;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0_usize;
    let mut counts = [0_usize; 26];
    let mut sum_per_char = [0_usize; 26];
    for (i, &c) in enumerate(&s) {
        let u = char_to_int(c);
        ans += sum_per_char[u] - counts[u] * (s.len() - i + 1);
        counts[u] += 1;
        sum_per_char[u] += s.len() - i;
    }

    println!("{}", ans);
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'A') as usize
}
