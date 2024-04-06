use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = (1..=n).map(|i| if i % 3 == 0 { 'x' } else { 'o' }).join("");
    println!("{}", ans);
}
