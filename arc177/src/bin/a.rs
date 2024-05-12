use itertools::izip;
use proconio::input;

const COINS: [usize; 6] = [1, 5, 10, 50, 100, 500];

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        mut numbers: [usize; 6],
        n: usize,
        xx: [usize; n],
    }

    for &x in &xx {
        let mut rem = x;

        for (coin, number) in izip!(COINS, &mut numbers).rev() {
            let use_num = (*number).min(rem / coin);
            *number -= use_num;
            rem -= coin * use_num;
        }

        if rem != 0 {
            return false;
        }
    }

    true
}
