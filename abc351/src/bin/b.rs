use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        aaa: [Chars; n],
        bbb: [Chars; n],
    }

    let (row, col) = iproduct!(0..n, 0..n)
        .find(|&(row, col)| aaa[row][col] != bbb[row][col])
        .unwrap();
    println!("{} {}", row + 1, col + 1);
}
