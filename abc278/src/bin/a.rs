use itertools::join;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut aa: [usize; n],
    }

    for _ in 0..k {
        aa.remove(0);
        aa.push(0);
    }

    println!("{}", join(aa, " "));
}
