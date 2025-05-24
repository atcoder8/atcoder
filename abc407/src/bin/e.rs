use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve()).join("\n"));
}

fn solve() -> usize {
    input! {
        n: usize,
        aa: [usize; 2 * n],
    }

    let mut sum = aa[0];
    let mut heap = BinaryHeap::<usize>::new();
    for i in 1..n {
        heap.extend([aa[2 * i - 1], aa[2 * i]]);
        sum += heap.pop().unwrap();
    }

    sum
}
