use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_by_key(|&a| Reverse(a));

    let mut iter = aa.clone().into_iter();
    let odd1 = iter.find(|&a| a % 2 == 1);
    let odd2 = iter.find(|&a| a % 2 == 1);

    let max_sum_odd = if let (Some(odd1), Some(odd2)) = (odd1, odd2) {
        Some(odd1 + odd2)
    } else {
        None
    };

    let mut iter = aa.into_iter();
    let even1 = iter.find(|&a| a % 2 == 0);
    let even2 = iter.find(|&a| a % 2 == 0);

    let max_sum_even = if let (Some(even1), Some(even2)) = (even1, even2) {
        Some(even1 + even2)
    } else {
        None
    };

    match (max_sum_odd, max_sum_even) {
        (None, None) => println!("-1"),
        (None, Some(even)) => println!("{}", even),
        (Some(odd), None) => println!("{}", odd),
        (Some(odd), Some(even)) => println!("{}", odd.max(even)),
    }
}
