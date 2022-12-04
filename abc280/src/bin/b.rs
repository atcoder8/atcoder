use itertools::{join, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [i64; n],
    }

    let mut aa = (0..(n - 1)).map(|i| ss[i + 1] - ss[i]).collect_vec();
    aa.insert(0, ss[0]);

    println!("{}", join(aa, " "));
}
