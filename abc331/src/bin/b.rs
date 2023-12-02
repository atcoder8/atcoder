use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (n, s, m, l): (usize, usize, usize, usize),
    }

    let mut ans = None;
    for (i, j, k) in iproduct!(0..=17, 0..=13, 0..=9) {
        if 6 * i + 8 * j + 12 * k >= n {
            let cost = s * i + m * j + l * k;
            if ans.is_none() || cost < ans.unwrap() {
                ans = Some(cost);
            }
        }
    }

    println!("{}", ans.unwrap());
}
