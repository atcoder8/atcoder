use itertools::Itertools;
use proconio::input;

fn main() {
    let mut aa = vec![];
    loop {
        input! {
            a: usize,
        }

        aa.push(a);

        if a == 0 {
            break;
        }
    }

    println!("{}", aa.iter().rev().join("\n"));
}
