use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        dd: [usize; n],
    }

    let is_ok = |digit: usize, day: usize| {
        let mut t = day + 1;

        while t != 0 {
            if t % 10 != digit {
                return false;
            }

            t /= 10;
        }

        true
    };

    let mut ans = 0;
    for month in 0..n {
        let mut digits = vec![];
        let mut t = month + 1;
        while t != 0 {
            digits.push(t % 10);
            t /= 10;
        }

        if !digits.iter().all_equal() {
            continue;
        }

        ans += (0..dd[month]).filter(|&day| is_ok(digits[0], day)).count();
    }

    println!("{}", ans);
}
