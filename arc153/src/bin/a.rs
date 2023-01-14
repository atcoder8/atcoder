use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let s = (n + 10_usize.pow(5) - 1).to_string().chars().collect_vec();
    let digits = s
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let seq = vec![0, 0, 1, 2, 3, 3, 4, 5, 4];

    let ans: usize = (0..9)
        .map(|i| digits[seq[i]] * 10_usize.pow(8 - i as u32))
        .sum();
    println!("{}", ans);
}
