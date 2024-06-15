use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, u32),
        ss: [String; n],
    }

    let bits_each_s = ss
        .iter()
        .map(|s| {
            enumerate(s.chars())
                .map(|(i, c)| ((c == 'o') as usize) << i)
                .sum::<usize>()
        })
        .collect_vec();

    let is_ok = |bits: usize| {
        let union = (0..n)
            .filter(|&i| bits >> i & 1 == 1)
            .fold(0, |acc, i| acc | bits_each_s[i]);

        if union.count_ones() == m {
            Some(bits.count_ones())
        } else {
            None
        }
    };

    let ans = (0..1 << n).filter_map(is_ok).min().unwrap();
    println!("{}", ans);
}
