use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k, x): (usize, usize, usize),
        mut aa: [usize; n],
    }

    aa.insert(k, x);

    println!("{}", aa.iter().join(" "));
}
