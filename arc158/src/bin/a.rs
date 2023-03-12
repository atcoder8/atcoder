use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        if let Some(ans) = solve() {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}

fn solve() -> Option<usize> {
    input! {
        xx: [usize; 3],
    }

    let sum_x: usize = xx.iter().sum();

    if !xx.iter().map(|&x| x % 2).all_equal() || sum_x % 3 != 0 {
        return None;
    }

    Some(xx.iter().map(|&x| abs_diff(x, sum_x / 3)).sum::<usize>() / 4)
}

fn abs_diff<T>(a: T, b: T) -> T
where
    T: PartialOrd<T> + std::ops::Sub<T, Output = T>,
{
    if a >= b {
        a - b
    } else {
        b - a
    }
}
