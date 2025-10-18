use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        t: usize,
        cd: [(usize, usize); t],
    }

    let output = cd.iter().map(|&(c, d)| solve(c, d)).join("\n");
    println!("{output}");
}

fn solve(c: usize, d: usize) -> usize {
    (1_u32..=10)
        .map(|num_digits| {
            let min = (c + 1).max(10_usize.pow(num_digits - 1));
            let max = (c + d).min(10_usize.pow(num_digits) - 1);
            if max >= min {
                concat_integer(c, max).sqrt() - (concat_integer(c, min) - 1).sqrt()
            } else {
                0
            }
        })
        .sum()
}

fn concat_integer(x: usize, y: usize) -> usize {
    (x.to_string() + &y.to_string()).parse().unwrap()
}
