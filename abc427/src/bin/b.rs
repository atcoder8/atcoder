use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![1];
    for i in 1..=n {
        let next = (0..i).map(|j| calc_digit_sum(dp[j])).sum();
        dp.push(next);
    }
    println!("{}", dp[n]);
}

fn calc_digit_sum(n: usize) -> usize {
    n.to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .sum()
}
