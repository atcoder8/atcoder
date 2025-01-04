use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (l, r): (usize, usize),
    }

    let ans = count_snake(r) - count_snake(l - 1);
    println!("{}", ans);
}

fn count_snake_with_head(n: usize, head: usize) -> usize {
    let digits = n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect_vec();
    let d0 = digits[0];

    let mut less = (head < d0) as usize;
    let mut equal = (head == d0) as usize;
    for &d in &digits[1..] {
        (less, equal) = (
            less * head + equal * head.min(d) + 1,
            equal * (d < head) as usize,
        );
    }

    less + equal - 1
}

fn count_snake(n: usize) -> usize {
    if n < 10 {
        return 0;
    }

    (1..=9).map(|head| count_snake_with_head(n, head)).sum()
}
