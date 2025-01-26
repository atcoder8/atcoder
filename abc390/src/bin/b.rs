use itertools::Itertools;
use num::Rational32;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i32; n],
    }

    let ans = aa
        .iter()
        .tuple_windows()
        .map(|(&a1, &a2)| Rational32::new(a2, a1))
        .all_equal();
    println!("{}", if ans { "Yes" } else { "No" });
}
