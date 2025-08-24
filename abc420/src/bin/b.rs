use itertools::{izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let mut scores = vec![0_usize; n];
    for turn in 0..m {
        let num_zeros = ss.iter().filter(|s| s[turn] == '0').count();
        let num_ones = n - num_zeros;

        if num_zeros == 0 || num_ones == 0 {
            scores.iter_mut().for_each(|score| *score += 1);
            continue;
        }

        let winner = if num_zeros < num_ones { '0' } else { '1' };
        for (score, s) in izip!(&mut scores, &ss) {
            *score += (s[turn] == winner) as usize;
        }
    }

    let max_score = *scores.iter().max().unwrap();

    let output = scores
        .iter()
        .positions(|&score| score == max_score)
        .map(|i| i + 1)
        .join(" ");
    println!("{output}");
}
