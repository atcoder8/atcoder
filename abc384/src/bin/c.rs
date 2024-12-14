use std::cmp::Reverse;

use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        scores: [usize; 5],
    }

    let chars = "ABCDE".chars().collect_vec();

    let bits_to_score = |bits: u8| {
        let mut sum_score = 0;
        for (i, &score) in enumerate(&scores) {
            if bits >> i & 1 == 1 {
                sum_score += score;
            }
        }

        sum_score
    };

    let bits_to_string = |bits: u8| {
        let mut s = String::new();
        for (i, &c) in enumerate(&chars) {
            if bits >> i & 1 == 1 {
                s.push(c);
            }
        }

        s
    };

    let ans = (1_u8..1 << scores.len())
        .sorted_unstable_by_key(|&bits| (Reverse(bits_to_score(bits)), bits_to_string(bits)))
        .map(bits_to_string)
        .join("\n");
    println!("{}", ans);
}
