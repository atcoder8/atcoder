use itertools::join;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (r, c): (usize, usize),
        bbb: [Chars; r],
    }

    let is_destroyed = |i: usize, j: usize| {
        for k in 0..r {
            for l in 0..c {
                if let Some(power) = bbb[k][l].to_digit(10) {
                    if abs_diff(i, k) + abs_diff(j, l) <= power as usize {
                        return true;
                    }
                }
            }
        }

        false
    };

    let mut ccc = bbb.clone();
    for i in 0..r {
        for j in 0..c {
            if bbb[i][j] != '#' || is_destroyed(i, j) {
                ccc[i][j] = '.'
            }
        }
    }

    let ans: Vec<String> = ccc.iter().map(|cc| cc.iter().collect()).collect();
    println!("{}", join(ans, "\n"));
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
