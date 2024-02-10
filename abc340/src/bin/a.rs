use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (a, b, d): (usize, usize, usize),
    }

    let ans = (a..).step_by(d).take_while(|&i| i <= b).join(" ");
    println!("{}", ans);
}
