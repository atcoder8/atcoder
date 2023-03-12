use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            xx: [usize; 3],
        }

        if let Some(ans) = solve(xx) {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}

fn solve(xx: Vec<usize>) -> Option<usize> {
    let sum_x: usize = xx.iter().sum();

    if !xx.iter().map(|&x| x % 2).all_equal() || sum_x % 3 != 0 {
        return None;
    }

    let a = sum_x / 3;

    Some(xx.iter().map(|&x| diff_abs(x, a)).sum::<usize>() / 4)
}

fn diff_abs(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}
