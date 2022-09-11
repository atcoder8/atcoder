use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let min_a = aa.iter().min().unwrap();
    let g = aa.iter().fold(0, |acc, a| acc.gcd(&(a - min_a)));

    println!("{}", if g == 1 { 2 } else { 1 });
}
