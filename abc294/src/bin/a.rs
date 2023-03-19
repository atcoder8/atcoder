use itertools::join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    println!("{}", join(aa.iter().filter(|&a| a % 2 == 0), " "));
}
