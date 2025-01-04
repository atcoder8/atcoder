use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let ans = iproduct!(1..=9, 1..=9)
        .filter_map(|(a, b)| {
            let mul = a * b;
            if mul != x {
                Some(mul)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("{}", ans);
}
