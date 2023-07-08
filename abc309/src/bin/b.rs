use itertools::join;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        aaa: [Chars; n],
    }

    println!("{}{}", aaa[1][0], join(&aaa[0][..(n - 1)], ""));
    for i in 1..(n - 1) {
        println!(
            "{}{}{}",
            aaa[i + 1][0],
            join(&aaa[i][1..(n - 1)], ""),
            aaa[i - 1][n - 1]
        );
    }
    println!("{}{}", join(&aaa[n - 1][1..], ""), aaa[n - 2][n - 1]);
}
