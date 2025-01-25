use itertools::Itertools;
use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let gcd = aa[0].gcd(&aa[1]);
    let denom = aa[0] / gcd;
    let numer = aa[1] / gcd;

    let ans = aa.iter().tuple_windows().all(|(&a1, &a2)| {
        let middle = a1 * numer;
        middle % denom == 0 && middle / denom == a2
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
