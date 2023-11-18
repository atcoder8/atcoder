use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [Usize1; m],
    }

    let mut counts = vec![0; n];
    let mut heap = BinaryHeap::new();
    for &a in &aa {
        counts[a] += 1;
        heap.push((counts[a], Reverse(a)));

        println!("{}", heap.peek().unwrap().1 .0 + 1);
    }
}
