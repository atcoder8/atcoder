// unfinished

use itertools::join;
use num_integer::Integer;
use proconio::input;

const COMPOSITE: usize = 735134400;

fn main() {
    input! {
        t: usize,
    }

    let divisors = find_divisors(COMPOSITE);

    for _ in 0..t {
        if let Some(ans) = solve(&divisors) {
            println!("Yes\n{}", join(ans, " "));
        } else {
            println!("No");
        }
    }
}

fn solve(divisors: &Vec<usize>) -> Option<Vec<usize>> {
    input! {
        n: usize,
    }

    if n == 1 {
        return Some(vec![1]);
    }

    if n == 2 {
        return None;
    }

    let mut aa = divisors[(divisors.len() - n + 1)..divisors.len()].to_owned();
    let lcm = aa.iter().fold(1, |acc, &x| acc.lcm(&x));
    let sum: usize = aa.iter().map(|&a| lcm / a).sum();
    let rem = lcm - sum;
    let gcd = rem.gcd(&lcm);
    assert_eq!(rem, gcd);
    aa.push(lcm / gcd);

    Some(aa)
}

/// Creates a sequence consisting of the divisors of `n`.
pub fn find_divisors(n: usize) -> Vec<usize> {
    assert_ne!(n, 0, "`n` must be at least 1.");

    let mut divisors = vec![];

    for i in (1..).take_while(|i| i * i <= n) {
        if n % i == 0 {
            divisors.push(i);

            if n / i != i {
                divisors.push(n / i);
            }
        }
    }

    divisors.sort_unstable();

    divisors
}
